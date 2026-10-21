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
    arguments = iter(command[command.index(subcommand) + 1:])
    values = list(arguments)
    if any(value in {"--dry-run", "--help", "-h", "--version", "-V"} for value in values):
        return "read-only"
    if subcommand != "clean":
        raise ValueError("anchored cargo sweep is unsupported; refusing pathname cleanup")
    arguments = iter(values)
    for value in arguments:
        if value in {"--manifest-path", "--target-dir", "--color"}:
            if next(arguments, None) is None:
                raise ValueError(f"missing value for {value}")
        elif value.startswith(("--manifest-path=", "--target-dir=", "--color=")):
            continue
        elif value not in {"-v", "-vv", "--verbose", "-q", "--quiet", "--locked", "--offline", "--frozen"}:
            raise ValueError(f"anchored selective cargo clean is unsupported: {value}")
    return "full"
