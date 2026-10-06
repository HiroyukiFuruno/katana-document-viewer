"""Prevent output directories from overlapping repository source files."""

import subprocess
from pathlib import Path


def reject_source_target(root: Path, target: Path) -> None:
    relative = target.relative_to(root)
    if ".git" in relative.parts:
        raise ValueError(f"target overlaps Git metadata: {target}")
    if root.joinpath(".git").exists():
        result = subprocess.run(
            ["git", "-C", str(root), "ls-files", "-z", "--", relative.as_posix()],
            capture_output=True, check=True,
        )
        if result.stdout:
            raise ValueError(f"target overlaps tracked source files: {target}")
    else:
        source = root / "src"
        if target == source or source in target.parents or target in source.parents:
            raise ValueError(f"target overlaps crate source: {target}")
    if target.joinpath("Cargo.toml").exists():
        raise ValueError(f"target contains a crate manifest: {target}")
