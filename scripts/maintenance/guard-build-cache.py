#!/usr/bin/env python3
"""Serialize guarded cargo runs and clean only owned, oversized KDV targets."""

from __future__ import annotations

import argparse
import errno
import math
import os
import stat
import subprocess
import sys
from pathlib import Path
from typing import Iterable

from cargo_target_args import command_directory, command_target_dirs
from cargo_output_directory import configured_target_directory
from target_ownership import CACHEDIR_TAG, prepare_targets, reject_source_target

try:
    import fcntl
except ImportError:
    fcntl = None
    import msvcrt

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


def repo_targets(
    repo_root: Path, explicit: Iterable[Path] = (), *, include_default: bool = True,
) -> list[Path]:
    root = repo_root.resolve()
    explicit = list(explicit)
    default = (root / "target").resolve()
    candidates = [default, *explicit] if include_default else explicit
    targets: list[Path] = []
    for candidate in candidates:
        path = candidate.resolve()
        if root not in path.parents:
            raise ValueError(f"target is outside the repository scope: {candidate}")
        if path.exists() and not path.is_dir():
            raise ValueError(f"target is not a directory: {path}")
        reject_source_target(root, path)
        targets.append(path)
    unique = sorted(set(targets), key=lambda item: (len(item.parts), str(item)))
    if include_default and default not in {path.resolve() for path in explicit} and not owned_target(default):
        if any(default in path.parents for path in unique):
            unique.remove(default)
    return unique


def owned_target(path: Path) -> bool:
    tag = path / "CACHEDIR.TAG"
    descriptor: int | None = None
    try:
        initial = tag.lstat()
        if not stat.S_ISREG(initial.st_mode):
            return False
        descriptor = _open_tag_descriptor(tag)
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode):
            return False
        with os.fdopen(descriptor, "r", encoding="utf-8") as stream:
            descriptor = None
            return stream.read(len(CACHEDIR_TAG)) == CACHEDIR_TAG
    except (OSError, UnicodeError):
        return False
    finally:
        if descriptor is not None:
            os.close(descriptor)


def _open_tag_descriptor(tag: Path) -> int:
    if os.name != "nt":
        nofollow = getattr(os, "O_NOFOLLOW", 0)
        if not nofollow:
            raise OSError(errno.EOPNOTSUPP, "O_NOFOLLOW is required")
        flags = os.O_RDONLY | os.O_NONBLOCK | nofollow
        return os.open(tag, flags)
    from windows_tag_handle import open_tag_descriptor

    return open_tag_descriptor(tag)


def target_bytes(path: Path, seen: set[tuple[int, int | str]] | None = None) -> int:
    total = 0
    if seen is None:
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
                    identity = (stat.st_dev, stat.st_ino or str(item.resolve()))
                    if identity in seen:
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
            if probe.stdout.strip():
                return True
            if probe.returncode not in (0, 1) or probe.stderr.strip():
                return True
    except FileNotFoundError as error:
        setup = "restore tasklist on Windows" if os.name == "nt" else "install lsof on POSIX before build-cache cleanup"
        print(f"required process inspection tool unavailable: {error.filename}; "
              f"{setup}", file=sys.stderr)
        return True
    except OSError:
        return True
    return False


def clean_targets(repo_root: Path, targets: list[Path]) -> None:
    from anchored_cleanup import purge_targets

    purge_targets(targets)


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


def try_acquire_lock(lock) -> bool:
    try:
        if fcntl is not None:
            fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        else:
            lock.seek(0, os.SEEK_END)
            if lock.tell() == 0:
                lock.write("0")
                lock.flush()
            lock.seek(0)
            msvcrt.locking(lock.fileno(), msvcrt.LK_NBLCK, 1)
        return True
    except OSError as error:
        if error.errno in {errno.EACCES, errno.EAGAIN, errno.EDEADLK}:
            return False
        raise


def guard(
    repo_root: Path, targets: list[Path], command: list[str], max_bytes: int,
    *, maintenance: bool = False,
) -> int:
    lock_path = repo_root / "tmp" / "maintenance" / "build-cache.lock"
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    with lock_path.open("a+") as lock:
        if maintenance:
            if not try_acquire_lock(lock):
                print("guarded cargo command is active; refusing maintenance", file=sys.stderr)
                return 3
        else:
            acquire_lock(lock)
        cleanup_mode = None
        if maintenance and cargo_subcommand(command) in {"clean", "sweep"}:
            from anchored_cleanup import maintenance_mode

            cleanup_mode = maintenance_mode(command, cargo_subcommand(command))
            if cleanup_mode == "read-only":
                return subprocess.run(command, check=False).returncode
        prepare_targets(repo_root, targets)
        seen: set[tuple[int, int | str]] = set()
        if maintenance or sum(target_bytes(target, seen) for target in targets) > max_bytes:
            if any(not owned_target(target) for target in targets if target.exists()):
                print("build cache ownership tag missing; refusing cleanup", file=sys.stderr)
                return 2
            if active_build(targets):
                print("active cargo build detected; refusing cleanup", file=sys.stderr)
                return 3
            if not maintenance:
                clean_targets(repo_root, targets)
            elif cleanup_mode == "full":
                clean_targets(repo_root, targets)
                return 0
        return subprocess.run(command, check=False).returncode

GUARDED_CARGO_SUBCOMMANDS = {
    "build", "test", "check", "clippy", "run", "bench", "llvm-cov", "package", "publish", "semver-checks"
}


def cargo_subcommand(command: list[str]) -> str | None:
    for index, value in enumerate(command[:-1]):
        if Path(value).name in {"cargo", "cargo.exe"}:
            arguments = iter(command[index + 1:])
            for argument in arguments:
                if argument in {"--color", "--config", "--manifest-path", "-C", "-Z"}:
                    next(arguments, None)
                elif not argument.startswith(("-", "+")):
                    return argument
            return None
    return None


def should_guard(command: list[str]) -> bool:
    return cargo_subcommand(command) in GUARDED_CARGO_SUBCOMMANDS


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
        subcommand = cargo_subcommand(command)
        maintenance = subcommand in {"clean", "sweep"}
        cargo_directory = command_directory(command)
        command_targets = command_target_dirs(command)
        cargo_target_dir = os.environ.get("CARGO_TARGET_DIR")
        if maintenance:
            if args.target_dir or len(command_targets) > 1:
                raise ValueError("maintenance requires one Cargo-selected target directory")
            if command_targets:
                selected = command_targets[0].resolve()
            elif cargo_target_dir is not None:
                if not cargo_target_dir:
                    raise ValueError("CARGO_TARGET_DIR must not be empty for maintenance")
                selected = (cargo_directory / cargo_target_dir).resolve()
            else:
                selected = (repo_root / "target").resolve()
            targets = repo_targets(repo_root, [selected], include_default=False)
            return guard(repo_root, targets, command, max_bytes, maintenance=True)
        explicit = [*(args.target_dir or ()), *command_targets]
        if cargo_target_dir:
            explicit.append((cargo_directory / cargo_target_dir).resolve())
        if should_guard(command) and not command_targets:
            explicit.append(configured_target_directory(command))
        targets = repo_targets(repo_root, explicit)
        if not should_guard(command):
            return subprocess.run(command, check=False).returncode
        return guard(repo_root, targets, command, max_bytes)
    except (OSError, RuntimeError, ValueError, subprocess.CalledProcessError) as error:
        print(f"build cache guard failed safely: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
