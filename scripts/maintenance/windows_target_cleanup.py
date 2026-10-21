"""Pin Windows target names and delete children through native handles."""

import ctypes
import errno
from contextlib import ExitStack, contextmanager
from ctypes import wintypes
from pathlib import Path

from windows_tag_handle import FileInformation, kernel_api, open_tag_descriptor

DIRECTORY = 0x10
REPARSE = 0x400
READONLY = 1
DELETE_ACCESS = 0x10000
READ_ATTRIBUTES = 0x80
LIST_DIRECTORY = 1
WRITE_ATTRIBUTES = 0x100


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
        self.active = False

    @contextmanager
    def _open(self, path: Path, *, delete: bool = False):
        # 属性だけのopenは共有競合検査へ参加しないため、pinには一覧読取も要求する。
        access = READ_ATTRIBUTES | (DELETE_ACCESS | WRITE_ATTRIBUTES if delete else LIST_DIRECTORY)
        # 全祖先と対象のrenameを禁止し、開いたHANDLE以外へ削除が逸れないようにする。
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
        import os

        try:
            for path in [*reversed(self.path.parents), self.path]:
                _, attributes = self.stack.enter_context(self._open(path))
                if not attributes & DIRECTORY or attributes & REPARSE:
                    raise OSError(errno.ELOOP, "cleanup ancestor is not a regular directory")
            descriptor = open_tag_descriptor(self.path / "CACHEDIR.TAG")
            stream = self.stack.enter_context(os.fdopen(descriptor, "rb"))
            expected = self.signature.encode("ascii")
            if stream.read(len(expected)) != expected:
                raise OSError(errno.EINVAL, "CACHEDIR.TAG signature mismatch")
        except BaseException:
            self.stack.close()
            raise
        self.active = True
        return self

    def __exit__(self, *error):
        self.active = False
        return self.stack.__exit__(*error)

    def _set(self, handle, kind, information):
        if not self.kernel.SetFileInformationByHandle(
            handle, kind, ctypes.byref(information), ctypes.sizeof(information),
        ):
            raise ctypes.WinError(ctypes.get_last_error())

    def _remove(self, path: Path):
        with self._open(path, delete=True) as (handle, attributes):
            if attributes & DIRECTORY and not attributes & REPARSE:
                for child in path.iterdir():
                    self._remove(child)
            if attributes & READONLY:
                basic = BasicInformation()
                basic.attributes = (attributes & ~READONLY) or 0x80
                self._set(handle, 0, basic)
            self._set(handle, 4, Disposition(True))

    def purge(self):
        if not self.active:
            raise RuntimeError("cleanup is not anchored")
        for child in self.path.iterdir():
            if child.name != "CACHEDIR.TAG":
                self._remove(child)
