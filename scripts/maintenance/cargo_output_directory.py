"""Ask Cargo to resolve configuration-relative output directories."""

import json
import os
import subprocess
from pathlib import Path


VALUE_OPTIONS = {"--config", "--manifest-path", "--color", "-C", "-Z"}
FLAG_OPTIONS = {"--locked", "--offline", "--frozen"}


def metadata_command(command: list[str]) -> list[str]:
    cargo_index = next(
        (index for index, value in enumerate(command) if Path(value).name in {"cargo", "cargo.exe"}),
        None,
    )
    if cargo_index is None:
        raise ValueError("Cargo executable is required to resolve its output directory")
    options: list[str] = []
    toolchain: list[str] = []
    arguments = iter(command[cargo_index + 1:])
    before_subcommand = True
    for argument in arguments:
        if argument == "--":
            break
        if argument in VALUE_OPTIONS:
            value = next(arguments, None)
            if value is None:
                raise ValueError(f"Cargo {argument} requires a value")
            options.extend((argument, value))
        elif argument in FLAG_OPTIONS or any(argument.startswith(option + "=") for option in VALUE_OPTIONS):
            options.append(argument)
        elif before_subcommand and argument.startswith("+"):
            toolchain.append(argument)
        elif argument.startswith(("-C", "-Z")):
            options.append(argument)
        elif before_subcommand and not argument.startswith("-"):
            before_subcommand = False
    return [*command[:cargo_index + 1], *toolchain, "metadata", *options, "--no-deps", "--format-version=1"]


def has_cargo_configuration(directory: Path) -> bool:
    if "CARGO_BUILD_TARGET_DIR" in os.environ:
        return True
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    if not cargo_home.is_absolute():
        cargo_home = directory / cargo_home
    config_directories = [cargo_home, *(parent / ".cargo" for parent in (directory, *directory.parents))]
    return any((parent / name).is_file() for parent in config_directories for name in ("config", "config.toml"))


def configured_target_directory(command: list[str], cwd: Path | None = None) -> Path:
    result = subprocess.run(metadata_command(command), cwd=cwd, check=True, capture_output=True, text=True)
    try:
        value = json.loads(result.stdout)["target_directory"]
    except (ValueError, KeyError, TypeError) as error:
        raise RuntimeError("Cargo metadata did not report its output directory") from error
    if not isinstance(value, str) or not value or not Path(value).is_absolute():
        raise RuntimeError("Cargo metadata output directory must be an absolute path")
    return Path(value).resolve()


def maintenance_target_directory(command: list[str], repo_root: Path, subcommand: str) -> Path:
    default = (repo_root / "target").resolve()
    if not has_cargo_configuration(repo_root):
        return default
    from anchored_cleanup import maintenance_mode

    if maintenance_mode(command, subcommand) == "read-only":
        return default
    return configured_target_directory(command, cwd=repo_root)
