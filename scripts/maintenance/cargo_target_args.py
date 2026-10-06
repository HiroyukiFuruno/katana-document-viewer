"""Read Cargo output directories without interpreting test binary arguments."""

from pathlib import Path


def command_target_dirs(command: list[str]) -> list[Path]:
    targets: list[Path] = []
    arguments = iter(command[1:])
    for argument in arguments:
        if argument == "--":
            break
        if argument in {"--color", "--config", "--manifest-path", "-C", "-Z"}:
            next(arguments, None)
        elif argument == "--target-dir":
            value = next(arguments, None)
            if value is None:
                raise ValueError("Cargo --target-dir requires a directory")
            targets.append(Path(value))
        elif argument.startswith("--target-dir="):
            value = argument.partition("=")[2]
            if not value:
                raise ValueError("Cargo --target-dir requires a directory")
            targets.append(Path(value))
    return targets
