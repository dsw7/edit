#!/usr/bin/env python3

from pathlib import Path
from shutil import copy


def main() -> None:
    path_plugin_dir = Path.home() / ".vim" / "plugin"
    path_plugin_dir.mkdir(exist_ok=True)

    script_dir = Path(__file__).parent
    path_copied_file = copy(script_dir / "edit.vim", path_plugin_dir)
    print(f"edit.vim -> {path_copied_file}")


if __name__ == "__main__":
    main()
