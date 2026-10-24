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
