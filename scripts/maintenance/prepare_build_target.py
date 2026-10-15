"""Claim only a newly created or empty, source-free Cargo output directory."""

from pathlib import Path

from target_ownership import prepare_empty_target


if __name__ == "__main__":
    root = Path(__file__).resolve().parents[2]
    prepare_empty_target(root, root / "target")
