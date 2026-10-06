# `vim` integration

## Setting up VimScript
Create a `vim` plugin directory:
```console
mkdir -vp ~/.vim/plugin
```
Then copy `edit.vim` to this directory:
```console
cp -v edit.vim ~/.vim/plugin/
```

## Usage
Select a block of text in visual mode, and execute `ed`. This will open the
selected text in a new vertical split window (named **Prompt**) with a new
scratch buffer. For example, selecting:
```c
void printIntAddr()
{
    const int i = 42;
    printf("Value: %d\n", i);
    printf("Address: %p\n", (void *)&i);
}
```
Will open:
```console
void printIntAddr()
{
    const int i = 42;
    printf("Value: %d\n", i);
    printf("Address: %p\n", (void *)&i);
}
─────────────────────────────────────────────────────────────────────────────────────────────────────────────

```
Place the instructions below the vertical separator and invoke `:W` to submit
the job. For example:
```console
void printIntAddr()
{
    const int i = 42;
    printf("Value: %d\n", i);
    printf("Address: %p\n", (void *)&i);
}
─────────────────────────────────────────────────────────────────────────────────────────────────────────────
What does the code do?
```
The results will be loaded into another window named **Completion** which will
be positioned beneath **Prompt**.
