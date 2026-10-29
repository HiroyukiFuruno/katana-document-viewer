"""Ask Cargo to resolve configuration-relative output directories."""

import json
import os
import subprocess
from pathlib import Path

from cargo_target_args import command_directory


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


def validate_configuration_paths(command: list[str]) -> None:
    # Cargoが読めない設定directoryを未設定扱いする前に権限エラーを顕在化する。
    directory = command_directory(command)
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    if not cargo_home.is_absolute():
        cargo_home = directory / cargo_home
    config_directories = [cargo_home, *(parent / ".cargo" for parent in (directory, *directory.parents))]
    for parent in config_directories:
        for name in ("config", "config.toml"):
            try:
                (parent / name).lstat()
            except FileNotFoundError:
                continue


def configured_output_directories(command: list[str], target_override: Path | None = None) -> list[Path]:
    validate_configuration_paths(command)
    environment = os.environ.copy()
    if target_override is not None:
        # metadataには--target-dirがないため、CLIで選ばれた値を環境へ束縛する。
        environment["CARGO_TARGET_DIR"] = str(target_override.resolve())
    result = subprocess.run(metadata_command(command), env=environment, check=True, capture_output=True, text=True)
    try:
        metadata = json.loads(result.stdout)
        values = [metadata[name] for name in ("target_directory", "build_directory")]
    except (ValueError, KeyError, TypeError) as error:
        raise RuntimeError("Cargo metadata did not report all output directories") from error
    if any(not isinstance(value, str) or not value or not Path(value).is_absolute() for value in values):
        raise RuntimeError("Cargo metadata output directories must be absolute paths")
    return list(dict.fromkeys(Path(value).resolve() for value in values))
