"""Pin Windows target names and delete children through native handles."""

import ctypes
import errno
from contextlib import ExitStack, contextmanager
from ctypes import wintypes
from pathlib import Path

from windows_tag_handle import FileInformation, kernel_api
from windows_relative_io import WindowsRelativeIo

DIRECTORY = 0x10
REPARSE = 0x400
READONLY = 1
READ_ATTRIBUTES = 0x80
LIST_DIRECTORY = 1


class BasicInformation(ctypes.Structure):
    _fields_ = [(name, ctypes.c_longlong) for name in ("created", "accessed", "written", "changed")]
    _fields_ += [("attributes", wintypes.DWORD)]


class Disposition(ctypes.Structure):
    _fields_ = [("delete", wintypes.BOOL)]


class WindowsTargetCleanup:
    """Hold every ancestor against renaming for the entire cleanup."""

    def __init__(self, path: Path, signature: str):
        self.path = path
        self.signature = signature
        self.kernel = kernel_api()
        self.kernel.SetFileInformationByHandle.restype = wintypes.BOOL
        self.kernel.SetFileInformationByHandle.argtypes = [
            wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD,
        ]
        self.stack = ExitStack()
        self.relative = WindowsRelativeIo(self.kernel)
        self.pins = []
        self.root_handle = None
        self.tag_handle = None
        self.active = False

    @contextmanager
    def _open_volume(self, path: Path):
        access = READ_ATTRIBUTES | LIST_DIRECTORY | 0x100000
        handle = self.kernel.CreateFileW(str(path), access, 1, None, 3, 0x02200000, None)
        if handle == ctypes.c_void_p(-1).value:
            raise ctypes.WinError(ctypes.get_last_error())
        try:
            info = FileInformation()
            if not self.kernel.GetFileInformationByHandle(handle, ctypes.byref(info)):
                raise ctypes.WinError(ctypes.get_last_error())
            yield handle, info.attributes
        finally:
            self.kernel.CloseHandle(handle)

    def __enter__(self):
        self.pins = []
        try:
            paths = [*reversed(self.path.parents), self.path]
            for index, path in enumerate(paths):
                context = self._open_volume(path) if index == 0 else self.relative.open_child(parent, path.name)
                parent, attributes = self.stack.enter_context(context)
                if not attributes & DIRECTORY or attributes & REPARSE:
                    raise OSError(errno.ELOOP, "cleanup ancestor is not a regular directory")
                self.pins.append(parent)
            self.root_handle = parent
            self.tag_handle, attributes = self.stack.enter_context(
                self.relative.open_child(parent, "CACHEDIR.TAG"))
            if attributes & (DIRECTORY | REPARSE):
                raise OSError(errno.ELOOP, "CACHEDIR.TAG is not a regular file")
            self.relative.read_signature(self.tag_handle, self.signature.encode("ascii"))
            self._check_pins()
        except BaseException:
            self.stack.close()
            raise
        self.active = True
        return self

    def __exit__(self, *error):
        self.active = False
        return self.stack.__exit__(*error)

    def _check_pins(self):
        for handle in self.pins:
            attributes = self.relative.attributes(handle)
            if not attributes & DIRECTORY or attributes & REPARSE:
                raise OSError(errno.ELOOP, "cleanup ancestor became a reparse point")
        if self.relative.attributes(self.tag_handle) & (DIRECTORY | REPARSE):
            raise OSError(errno.ELOOP, "CACHEDIR.TAG became a reparse point")

    def _set(self, handle, kind, information):
        if not self.kernel.SetFileInformationByHandle(
            handle, kind, ctypes.byref(information), ctypes.sizeof(information),
        ):
            raise ctypes.WinError(ctypes.get_last_error())

    def _remove(self, parent, name):
        with self.relative.open_child(parent, name, delete=True) as (handle, attributes):
            self._remove_open(handle, attributes)

    def _remove_open(self, handle, attributes):
        if attributes & DIRECTORY and not attributes & REPARSE:
            for name in list(self.relative.entries(handle)):
                self._remove(handle, name)
        if attributes & READONLY:
            basic = BasicInformation()
            basic.attributes = (attributes & ~READONLY) or 0x80
            self._set(handle, 0, basic)
        self._set(handle, 4, Disposition(True))

    def purge(self):
        if not self.active:
            raise RuntimeError("cleanup is not anchored")
        self._check_pins()
        for name in list(self.relative.entries(self.root_handle)):
            if name.casefold() != "cachedir.tag":
                self._remove(self.root_handle, name)
        self._check_pins()
