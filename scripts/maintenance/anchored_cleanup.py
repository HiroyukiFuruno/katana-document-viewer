"""Use anchored full purges; refuse unsupported destructive selectors."""

import os
from contextlib import ExitStack
from pathlib import Path

from target_ownership import CACHEDIR_TAG


def purge_targets(targets: list[Path]) -> None:
    if os.name == "nt":
        from windows_target_cleanup import WindowsTargetCleanup as Cleanup
    else:
        from posix_target_cleanup import PosixTargetCleanup as Cleanup
    # 入れ子targetの親を消してから子のpathnameを開き直さない。
    roots = [p for p in targets if not any(other in p.parents for other in targets)]
    with ExitStack() as stack:
        anchors = [stack.enter_context(Cleanup(path, CACHEDIR_TAG)) for path in roots if path.exists()]
        for anchor in anchors:
            anchor.purge()


def maintenance_mode(command: list[str], subcommand: str) -> str:
    """Return full/read-only mode, refusing selectors without changing their meaning."""
    cargo_index = next(i for i, value in enumerate(command) if Path(value).name in {"cargo", "cargo.exe"})
    subcommand_index = cargo_index + 1
    if command[subcommand_index] != subcommand:
        raise ValueError("anchored maintenance does not support Cargo global selectors")
    values = command[subcommand_index + 1:]
    if any(value in {"--dry-run", "-n", "--help", "-h", "--version", "-V"} for value in values):
        return "read-only"
    if subcommand != "clean":
        raise ValueError("anchored cargo sweep is unsupported; refusing pathname cleanup")
    arguments = iter(values)
    target_seen = False
    for value in arguments:
        if value in {"--target-dir", "--color"}:
            argument = next(arguments, None)
            if not argument or argument.startswith("-"):
                raise ValueError(f"missing value for {value}")
            if value == "--color" and argument not in {"always", "never", "auto"}:
                raise ValueError("invalid Cargo color mode")
        elif value.startswith(("--target-dir=", "--color=")):
            option, _, argument = value.partition("=")
            if not argument or (option == "--color" and argument not in {"always", "never", "auto"}):
                raise ValueError(f"invalid value for {option}")
        elif value not in {"-v", "-vv", "--verbose", "-q", "--quiet", "--locked", "--offline", "--frozen"}:
            raise ValueError(f"anchored selective cargo clean is unsupported: {value}")
        if value == "--target-dir" or value.startswith("--target-dir="):
            if target_seen:
                raise ValueError("repeated Cargo target selection is unsupported")
            target_seen = True
    return "full"
