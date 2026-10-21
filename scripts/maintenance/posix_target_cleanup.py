"""Remove an anchored POSIX Cargo target without following pathname swaps."""

from __future__ import annotations

import errno
import os
import shutil
import stat
from pathlib import Path


class PosixTargetCleanup:
    """Keep a directory FD while deleting only its anchored children."""

    def __init__(self, path: Path, signature: str) -> None:
        self.path = path
        self.signature = signature
        self._fd: int | None = None

    def __enter__(self) -> "PosixTargetCleanup":
        directory_flag = getattr(os, "O_DIRECTORY", 0)
        nofollow = getattr(os, "O_NOFOLLOW", 0)
        if not directory_flag or not nofollow or not getattr(shutil.rmtree, "avoids_symlink_attacks", False):
            raise OSError(errno.EOPNOTSUPP, "anchored POSIX cleanup is unsupported")
        if not self.path.is_absolute() or ".." in self.path.parts:
            raise ValueError("cleanup requires an absolute normalized directory")
        flags = os.O_RDONLY | directory_flag | nofollow
        fd = os.open(self.path.anchor, flags)
        try:
            # 最終要素だけでなく祖先の差替えでも外部pathを辿らない。
            for component in self.path.parts[1:]:
                child_fd = os.open(component, flags, dir_fd=fd)
                os.close(fd)
                fd = child_fd
            self._verify_signature(fd)
        except BaseException:
            os.close(fd)
            raise
        self._fd = fd
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        if self._fd is not None:
            os.close(self._fd)
            self._fd = None

    def purge(self) -> None:
        if self._fd is None:
            raise RuntimeError("cleanup is not anchored")
        for name in os.listdir(self._fd):
            if name == "CACHEDIR.TAG":
                continue
            entry = os.stat(name, dir_fd=self._fd, follow_symlinks=False)
            if stat.S_ISDIR(entry.st_mode) and not stat.S_ISLNK(entry.st_mode):
                shutil.rmtree(name, dir_fd=self._fd)
            else:
                os.unlink(name, dir_fd=self._fd)
        self._verify_signature(self._fd)

    def _verify_signature(self, root_fd: int) -> None:
        nofollow = getattr(os, "O_NOFOLLOW", 0)
        if not nofollow:
            raise OSError(errno.EOPNOTSUPP, "tag no-follow is unsupported")
        flags = os.O_RDONLY | os.O_NONBLOCK | nofollow
        tag_fd = os.open("CACHEDIR.TAG", flags, dir_fd=root_fd)
        try:
            if not stat.S_ISREG(os.fstat(tag_fd).st_mode):
                raise OSError(errno.EINVAL, "CACHEDIR.TAG is not a regular file")
            expected = self.signature.encode("ascii")
            actual = os.read(tag_fd, len(expected))
            if actual != expected:
                raise OSError(errno.EINVAL, "CACHEDIR.TAG signature mismatch")
        finally:
            os.close(tag_fd)
