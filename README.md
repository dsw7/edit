# `edit`

Edit code using Anthropic LLMs from inside `vim`.

# Why?
Claude consumes tens of thousands of tokens before even reasoning with the
code, which I find unacceptably wasteful for smaller tasks. This program
bypasses this overhead by operating on individual `vim` selections directly via
Anthropic's API.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)

## Prerequisites
This program requires a valid Anthropic API key. Set it as an environment
variable:
```bash
export ANTHROPIC_API_KEY="<your-api-key>"
```

## Installation

### Step 1. Install executable
Run:
```console
make install
```
Then confirm that the executable is located in `$PATH`:
```console
edit --version
```

### Step 2. Initialize the program
Run:
```console
edit init
```
This command will create the program's application directory and other
necessary files. Edit the configuration file to suit your needs.

### Step 3. Install `vim` plugin
Run the setup script:
```console
./vim/setup.py
```

## Usage
Select a block of text in visual mode, and execute `ed`. This will open the
selected text in a new vertical split window (named **Prompt**). For example,
selecting:
```python3
def main() -> None:
    path_plugin_dir = Path.home() / ".vim" / "plugin"
    path_plugin_dir.mkdir(exist_ok=True)

    path_copied_file = copy("edit.vim", path_plugin_dir)
    print(f"edit.vim -> {path_copied_file}")
```
Will open:
```plaintext
def main() -> None:
    path_plugin_dir = Path.home() / ".vim" / "plugin"
    path_plugin_dir.mkdir(exist_ok=True)

    path_copied_file = copy("edit.vim", path_plugin_dir)
    print(f"edit.vim -> {path_copied_file}")
─────────────────────────────────────────────────────────────────────────────────────────────────────────────
>
```
Place the instructions below the vertical separator and hit <kbd>Enter</kbd> to
submit the job. For example:
```plaintext
def main() -> None:
    path_plugin_dir = Path.home() / ".vim" / "plugin"
    path_plugin_dir.mkdir(exist_ok=True)

    path_copied_file = copy("edit.vim", path_plugin_dir)
    print(f"edit.vim -> {path_copied_file}")
─────────────────────────────────────────────────────────────────────────────────────────────────────────────
> `copy` looks for `"edit.vim"` in the current working directory. change this so that `copy` looks for the file relative to the script calling the `main` function
```
The results will be loaded into another window named **Completion** which will
be positioned beneath **Prompt**. The token usage, inferred language, and model
will also be reported.
