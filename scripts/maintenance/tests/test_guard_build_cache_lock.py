import errno
import tempfile
import unittest
from unittest import mock

from test_guard_build_cache import guard


class WindowsLockTests(unittest.TestCase):
    def test_windows_lock_contention_waits_until_owner_finishes(self):
        api = mock.Mock()
        api.locking.side_effect = [OSError(errno.EACCES, "busy"), None]
        with tempfile.TemporaryFile(mode="w+") as lock:
            with mock.patch.object(guard, "fcntl", None):
                with mock.patch.object(guard, "msvcrt", api, create=True):
                    guard.acquire_lock(lock)
        self.assertEqual(2, api.locking.call_count)

    def test_windows_invalid_lock_descriptor_is_not_retried(self):
        api = mock.Mock()
        api.locking.side_effect = OSError(errno.EBADF, "invalid descriptor")
        with tempfile.TemporaryFile(mode="w+") as lock:
            with mock.patch.object(guard, "fcntl", None):
                with mock.patch.object(guard, "msvcrt", api, create=True):
                    with self.assertRaises(OSError):
                        guard.acquire_lock(lock)
        self.assertEqual(1, api.locking.call_count)


if __name__ == "__main__":
    unittest.main()
