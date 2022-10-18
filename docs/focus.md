# focus

A more comprehensive tool than `focus-send`, with features that would break the
goal of being compatible with Kaleidoscope's `bin/focus-send`. The tool provides
a number of sub-commands, all of which share a common set of options (unless
stated otherwise):

## Shared options

- `-d`, `--device` `<PATH>`: The device to connect to. If not specified, the
  tool will find all supported devices, and connect to the first found.
- `-c`, `--chunk-size` `<CHUNK_SIZE>`: Sets the chunk size to use when sending data. Defaults to 32, the same as Chrysalis. Setting the chunk size to zero will disable chunking, and all data will be written in one go.
- `-q`, `--quiet`: The tool displays a progress indicator by default. If this
  option is specified, it will not display one.

## Commands

### `help`

Prints a help screen.

Does not support the shared options, but can be given a sub-command, in which
case it will print the help screen of the sub-command, rather than the tools
own.

### `list-ports`

Lists port paths that belong to auto-detected supported devices.

Does not support the shared options.

### `send <COMMAND> [<ARGUMENTS...>]`

Send the given `<COMMAND>` to the device, wait for, and then display the reply.
The `<COMMAND>` is mandatory, `<ARGUMENTS...>` are optional.

### `backup`

Reads every setting from the keyboard, and outputs a JSON-formatted backup to
standard output. The output can be fed back to the `restore` command.

### `restore`

Reads a JSON-formatted backup from the standard input, and restores the settings
stores within it onto the keyboard.
