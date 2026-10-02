"""Keep repository-local hook settings out of self-test Git fixtures."""

from __future__ import annotations

import contextlib
import os
import subprocess
from collections.abc import Iterator


@contextlib.contextmanager
def isolated_git_fixture() -> Iterator[None]:
    names = subprocess.run(
        ["git", "rev-parse", "--local-env-vars"], check=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    ).stdout.splitlines()
    previous = {name: os.environ[name] for name in names if name in os.environ}
    # hookのGIT_DIRはcwdより優先され、fixture初期化が呼び出し元を変更する。
    try:
        for name in names:
            os.environ.pop(name, None)
        yield
    finally:
        for name in names:
            os.environ.pop(name, None)
        os.environ.update(previous)
