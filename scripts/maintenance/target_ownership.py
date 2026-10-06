"""Prevent output directories from overlapping repository source files."""

import subprocess
from pathlib import Path

CACHEDIR_TAG = "Signature: 8a477f597d28d172789f06886806bc55"


def prepare_targets(root: Path, targets: list[Path]) -> None:
    for target in targets:
        prepare_empty_target(root, target)


def prepare_empty_target(root: Path, target: Path) -> None:
    root, target = root.resolve(), target.resolve()
    reject_source_target(root, target)
    target.mkdir(parents=True, exist_ok=True)
    if any(target.iterdir()):
        return
    # cache復元や補助ツールが先にdirectoryを作るとCargoは所有タグを生成しない。
    with (target / "CACHEDIR.TAG").open("x", encoding="utf-8") as tag:
        tag.write(CACHEDIR_TAG + "\n# KDV Cargo output directory\n")


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
