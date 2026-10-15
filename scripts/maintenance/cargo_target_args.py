"""Read Cargo output directories without interpreting test binary arguments."""

from pathlib import Path


def command_directory(command: list[str], cwd: Path | None = None) -> Path:
    directory = (cwd or Path.cwd()).resolve()
    arguments = iter(command[1:])
    for argument in arguments:
        if argument == "--":
            break
        if argument == "-C":
            value = next(arguments, None)
            if value is None or value.startswith("-"):
                raise ValueError("Cargo -C requires a directory")
            directory = (directory / value).resolve()
        elif argument.startswith("-C") and len(argument) > 2:
            directory = (directory / argument[2:]).resolve()
        elif argument in {"--color", "--config", "-Z"}:
            next(arguments, None)
        elif not argument.startswith(("-", "+")):
            break
    return directory


def command_target_dirs(command: list[str], cwd: Path | None = None) -> list[Path]:
    targets: list[Path] = []
    initial_directory = (cwd or Path.cwd()).resolve()
    directory = command_directory(command, cwd)
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
            target = Path(value)
            targets.append((directory / target).resolve() if directory != initial_directory else target)
        elif argument.startswith("--target-dir="):
            value = argument.partition("=")[2]
            if not value:
                raise ValueError("Cargo --target-dir requires a directory")
            target = Path(value)
            targets.append((directory / target).resolve() if directory != initial_directory else target)
    return targets
