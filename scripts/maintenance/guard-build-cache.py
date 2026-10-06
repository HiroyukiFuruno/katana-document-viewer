#!/usr/bin/env python3
"""Serialize guarded cargo runs and clean only owned, oversized KDV targets."""

from __future__ import annotations

import argparse
import errno
import math
import os
import subprocess
import sys
from pathlib import Path
from typing import Iterable

try:
    import fcntl
except ImportError:
    fcntl = None
    import msvcrt

CACHEDIR_TAG = "Signature: 8a477f597d28d172789f06886806bc55"
DEFAULT_MAX_GIB = 40.0


def parse_max_gib(value: str | None = None) -> int:
    raw = value if value is not None else os.environ.get("KDV_BUILD_CACHE_MAX_GIB", str(DEFAULT_MAX_GIB))
    try:
        gib = float(raw)
    except ValueError as error:
        raise ValueError("KDV_BUILD_CACHE_MAX_GIB must be a positive number") from error
    if not math.isfinite(gib) or gib <= 0:
        raise ValueError("KDV_BUILD_CACHE_MAX_GIB must be a positive number")
    return math.ceil(gib * 1024**3)


def repo_targets(repo_root: Path, explicit: Iterable[Path] = ()) -> list[Path]:
    root = repo_root.resolve()
    candidates = [root / "target", *explicit]
    targets: list[Path] = []
    for candidate in candidates:
        path = candidate.resolve()
        if root not in path.parents or path.name != "target" and "target" not in path.parts:
            raise ValueError(f"target is outside the repository scope: {candidate}")
        if path.exists() and not path.is_dir():
            raise ValueError(f"target is not a directory: {path}")
        targets.append(path)
    unique: list[Path] = []
    for path in sorted(set(targets), key=lambda item: (len(item.parts), str(item))):
        if not any(parent in path.parents for parent in unique):
            unique.append(path)
    return unique


def owned_target(path: Path) -> bool:
    tag = path / "CACHEDIR.TAG"
    try:
        return tag.is_file() and tag.read_text(encoding="utf-8").startswith(CACHEDIR_TAG)
    except OSError:
        return False


def target_bytes(path: Path) -> int:
    total = 0
    seen = set()
    if not path.exists():
        return 0
    for current, directories, files in os.walk(path, followlinks=False):
        directories[:] = [name for name in directories if not Path(current, name).is_symlink()]
        for name in files:
            item = Path(current, name)
            if not item.is_symlink():
                try:
                    stat = item.stat()
                    identity = (stat.st_dev, stat.st_ino)
                    if stat.st_ino and identity in seen:
                        continue
                    seen.add(identity)
                    total += stat.st_blocks * 512 if hasattr(stat, "st_blocks") else stat.st_size
                except OSError:
                    raise RuntimeError(f"cannot inspect build cache entry: {item}")
    return total


def active_build(targets: list[Path]) -> bool:
    try:
        if os.name == "nt":
            processes = subprocess.run(["tasklist", "/fo", "csv", "/nh"], check=False, capture_output=True, text=True)
            if processes.returncode != 0:
                return True
            output = processes.stdout.lower()
            return '"cargo.exe"' in output or '"rustc.exe"' in output
        for target in targets:
            if not target.exists():
                continue
            probe = subprocess.run(["lsof", "+D", str(target)], check=False, capture_output=True, text=True)
            if probe.returncode == 0 and probe.stdout.strip():
                return True
            if probe.returncode not in (0, 1) or probe.stderr.strip():
                return True
    except OSError:
        return True
    return False


def clean_targets(repo_root: Path, targets: list[Path]) -> None:
    manifest = repo_root / "Cargo.toml"
    for target in targets:
        subprocess.run(
            ["cargo", "clean", "--manifest-path", str(manifest), "--target-dir", str(target)],
            check=True,
        )


def acquire_lock(lock) -> None:
    if fcntl is not None:
        fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
        return
    lock.seek(0)
    lock.write("0")
    lock.flush()
    lock.seek(0)
    while True:
        try:
            msvcrt.locking(lock.fileno(), msvcrt.LK_LOCK, 1)
            return
        except OSError as error:
            if error.errno not in {errno.EACCES, errno.EDEADLK}:
                raise


def guard(repo_root: Path, targets: list[Path], command: list[str], max_bytes: int) -> int:
    lock_path = repo_root / "tmp" / "maintenance" / "build-cache.lock"
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    with lock_path.open("a+") as lock:
        acquire_lock(lock)
        total = sum(target_bytes(target) for target in targets)
        if total > max_bytes:
            if any(not owned_target(target) for target in targets if target.exists()):
                print("build cache ownership tag missing; refusing cleanup", file=sys.stderr)
                return 2
            if active_build(targets):
                print("active cargo build detected; refusing cleanup", file=sys.stderr)
                return 3
            clean_targets(repo_root, targets)
        return subprocess.run(command, check=False).returncode


GUARDED_CARGO_SUBCOMMANDS = {
    "build", "test", "check", "clippy", "run", "bench", "llvm-cov", "package", "publish", "semver-checks"
}


def should_guard(command: list[str]) -> bool:
    for index, value in enumerate(command[:-1]):
        if Path(value).name in {"cargo", "cargo.exe"}:
            arguments = iter(command[index + 1:])
            for argument in arguments:
                if argument in {"--color", "--config", "--manifest-path", "-C", "-Z"}:
                    next(arguments, None)
                elif not argument.startswith(("-", "+")):
                    return argument in GUARDED_CARGO_SUBCOMMANDS
            return False
    return False


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--target-dir", action="append", type=Path)
    parser.add_argument("--max-gib")
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    if not command:
        parser.error("a wrapped command is required after --")
    try:
        max_bytes = parse_max_gib(args.max_gib)
        repo_root = args.repo_root.resolve()
        explicit = list(args.target_dir or ())
        cargo_target_dir = os.environ.get("CARGO_TARGET_DIR")
        if cargo_target_dir:
            explicit.append(Path(cargo_target_dir))
        targets = repo_targets(repo_root, explicit)
        if not should_guard(command):
            return subprocess.run(command, check=False).returncode
        return guard(repo_root, targets, command, max_bytes)
    except (OSError, RuntimeError, ValueError, subprocess.CalledProcessError) as error:
        print(f"build cache guard failed safely: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
