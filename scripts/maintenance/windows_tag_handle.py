"""Open Windows ownership markers without following reparse points."""

import ctypes
import errno
import os
from ctypes import wintypes
from pathlib import Path


class FileInformation(ctypes.Structure):
    _fields_ = [
        ("attributes", wintypes.DWORD),
        ("created", wintypes.FILETIME),
        ("accessed", wintypes.FILETIME),
        ("written", wintypes.FILETIME),
        ("volume", wintypes.DWORD),
        ("size_high", wintypes.DWORD),
        ("size_low", wintypes.DWORD),
        ("links", wintypes.DWORD),
        ("index_high", wintypes.DWORD),
        ("index_low", wintypes.DWORD),
    ]


def kernel_api():
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel.CreateFileW.restype = wintypes.HANDLE
    kernel.CreateFileW.argtypes = [
        wintypes.LPCWSTR, wintypes.DWORD, wintypes.DWORD, ctypes.c_void_p,
        wintypes.DWORD, wintypes.DWORD, wintypes.HANDLE,
    ]
    kernel.GetFileType.restype = wintypes.DWORD
    kernel.GetFileType.argtypes = [wintypes.HANDLE]
    kernel.GetFileInformationByHandle.restype = wintypes.BOOL
    kernel.GetFileInformationByHandle.argtypes = [wintypes.HANDLE, ctypes.POINTER(FileInformation)]
    kernel.CloseHandle.restype = wintypes.BOOL
    kernel.CloseHandle.argtypes = [wintypes.HANDLE]
    return kernel


def require_regular_file(kernel, handle):
    if kernel.GetFileType(handle) != 1:
        raise OSError(errno.ELOOP, "tag is not a disk file")
    information = FileInformation()
    if not kernel.GetFileInformationByHandle(handle, ctypes.byref(information)):
        raise ctypes.WinError(ctypes.get_last_error())
    if information.attributes & (0x10 | 0x400):
        raise OSError(errno.ELOOP, "tag is a directory or reparse point")


def open_tag_descriptor(tag: Path) -> int:
    import msvcrt

    kernel = kernel_api()
    # 読み取り中の削除・置換を共有せず、リンク自体を検査する。
    handle = kernel.CreateFileW(str(tag), 0x80000000, 1, None, 3, 0x00200000, None)
    if handle == ctypes.c_void_p(-1).value:
        raise ctypes.WinError(ctypes.get_last_error())
    transferred = False
    try:
        require_regular_file(kernel, handle)
        descriptor = msvcrt.open_osfhandle(handle, os.O_RDONLY | os.O_BINARY)
        transferred = True
        return descriptor
    finally:
        if not transferred:
            kernel.CloseHandle(handle)
