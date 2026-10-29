"""保持したWindows directory HANDLEだけを基準に子を扱う。"""

import ctypes
import errno
import struct
from contextlib import contextmanager
from ctypes import wintypes

from windows_tag_handle import FileInformation

READ_DATA = 1
READ_ATTRIBUTES = 0x80
WRITE_ATTRIBUTES = 0x100
DELETE_ACCESS = 0x10000
SYNCHRONIZE = 0x100000


class UnicodeString(ctypes.Structure):
    _fields_ = [("length", ctypes.c_uint16), ("maximum", ctypes.c_uint16),
                ("buffer", ctypes.c_void_p)]


class ObjectAttributes(ctypes.Structure):
    _fields_ = [("length", ctypes.c_uint32), ("root", wintypes.HANDLE),
                ("name", ctypes.POINTER(UnicodeString)), ("attributes", ctypes.c_uint32),
                ("security", ctypes.c_void_p), ("quality", ctypes.c_void_p)]


class StatusPointer(ctypes.Union):
    _fields_ = [("status", ctypes.c_int32), ("pointer", ctypes.c_void_p)]


class IoStatus(ctypes.Structure):
    _fields_ = [("value", StatusPointer), ("information", ctypes.c_size_t)]


class WindowsRelativeIo:
    def __init__(self, kernel):
        self.kernel = kernel
        self.nt = ctypes.WinDLL("ntdll", use_last_error=True)
        self.nt.NtCreateFile.restype = ctypes.c_int32
        self.nt.NtCreateFile.argtypes = [
            ctypes.POINTER(wintypes.HANDLE), ctypes.c_uint32, ctypes.POINTER(ObjectAttributes),
            ctypes.POINTER(IoStatus), ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint32,
            ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p, ctypes.c_uint32,
        ]
        self.nt.NtQueryDirectoryFile.restype = ctypes.c_int32
        self.nt.NtQueryDirectoryFile.argtypes = [
            wintypes.HANDLE, wintypes.HANDLE, ctypes.c_void_p, ctypes.c_void_p,
            ctypes.POINTER(IoStatus), ctypes.c_void_p, ctypes.c_uint32, ctypes.c_int32,
            ctypes.c_ubyte, ctypes.c_void_p, ctypes.c_ubyte,
        ]
        self.nt.RtlNtStatusToDosError.restype = ctypes.c_uint32
        self.nt.RtlNtStatusToDosError.argtypes = [ctypes.c_int32]
        self.kernel.ReadFile.restype = wintypes.BOOL
        self.kernel.ReadFile.argtypes = [wintypes.HANDLE, ctypes.c_void_p, wintypes.DWORD,
                                        ctypes.POINTER(wintypes.DWORD), ctypes.c_void_p]

    def _error(self, status):
        return ctypes.WinError(self.nt.RtlNtStatusToDosError(status))

    def attributes(self, handle):
        info = FileInformation()
        if not self.kernel.GetFileInformationByHandle(handle, ctypes.byref(info)):
            raise ctypes.WinError(ctypes.get_last_error())
        return info.attributes

    @contextmanager
    def open_child(self, parent, name, *, delete=False):
        if not name or name in {".", ".."} or any(value in name for value in ("/", "\\", ":", "\0")):
            raise OSError(errno.EINVAL, "cleanup child must be a single component")
        encoded = name.encode("utf-16-le")
        if len(encoded) > 65532:
            raise OSError(errno.ENAMETOOLONG, "cleanup child name is too long")
        buffer = ctypes.create_string_buffer(encoded + b"\0\0")
        name_value = UnicodeString(len(encoded), len(encoded) + 2, ctypes.addressof(buffer))
        # RootDirectoryと一成分名で、検証後に変わるancestor pathnameを再解決しない。
        attributes = ObjectAttributes(ctypes.sizeof(ObjectAttributes), parent,
                                      ctypes.pointer(name_value), 0x1040, None, None)
        access = READ_DATA | READ_ATTRIBUTES | SYNCHRONIZE
        if delete:
            access |= DELETE_ACCESS | WRITE_ATTRIBUTES
        handle, status = wintypes.HANDLE(), IoStatus()
        result = self.nt.NtCreateFile(ctypes.byref(handle), access, ctypes.byref(attributes),
                                      ctypes.byref(status), None, 0, 1, 1, 0x200020, None, 0)
        if result < 0:
            raise self._error(result)
        try:
            yield handle.value, self.attributes(handle)
        finally:
            self.kernel.CloseHandle(handle)

    def entries(self, handle):
        buffer = ctypes.create_string_buffer(65536)
        restart = True
        while True:
            status = IoStatus()
            result = self.nt.NtQueryDirectoryFile(handle, None, None, None, ctypes.byref(status),
                                                  buffer, len(buffer), 12, True, None, restart)
            restart = False
            if result & 0xFFFFFFFF == 0x80000006:
                return
            if result != 0:
                raise self._error(result)
            length = status.information
            if length < 12 or length > len(buffer):
                raise OSError(errno.EIO, "invalid directory information length")
            next_offset, _, name_length = struct.unpack_from("<III", buffer.raw)
            if next_offset or name_length % 2 or name_length > length - 12:
                raise OSError(errno.EIO, "invalid directory name information")
            name = buffer.raw[12:12 + name_length].decode("utf-16-le")
            if name not in {".", ".."}:
                yield name

    def read_signature(self, handle, expected):
        buffer = ctypes.create_string_buffer(len(expected))
        count = wintypes.DWORD()
        if not self.kernel.ReadFile(handle, buffer, len(expected), ctypes.byref(count), None):
            raise ctypes.WinError(ctypes.get_last_error())
        if count.value != len(expected) or buffer.raw != expected:
            raise OSError(errno.EINVAL, "CACHEDIR.TAG signature mismatch")
