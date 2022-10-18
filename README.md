# kaleidoscope-focus.rs

> Rust library (& CLI tools) to interface with [Kaleidoscope][kaleidoscope]
> powered keyboards via [Focus][focus].

 [focus]: https://kaleidoscope.readthedocs.io/en/latest/plugins/Kaleidoscope-FocusSerial.html

## Downloads

[![Latest development builds][badge:development]][build:dev]

 [badge:development]: https://img.shields.io/github/v/release/keyboardio/kaleidoscope-focus.rs?include_prereleases&label=Development&style=for-the-badge
 [build:dev]: https://github.com/keyboardio/kaleidoscope-focus.rs/releases/tag/v0.1.0-snapshot

## Included tools

### `focus-send`

A mostly faithful and compatible port of the [bin/focus-send][k:focus-send] tool
in Kaleidoscope to Rust. It's interface compatible: takes the same input, and
produces the same output. The major difference is that this version of the tool
writes data in 32-byte chunks (like Chrysalis), and can auto-detect the device
to use.

 [k:focus-send]: https://github.com/keyboardio/Kaleidoscope/blob/master/bin/focus-send

Otherwise, the usage is simple: `focus-send COMMAND ARGUMENTS...`

The `COMMAND` is the Focus command to send, with optional arguments. In case
there are multiple supported devices, the tool defaults to using the first one.
If that is not desirable, the `--device` (or `-d`) argument can be used to
specify the device to connect to. To remain compatible with Kaleidoscope's
`bin/focus-send`, we can also use the `DEVICE` environment variable for the same
purpose.

### `focus`

A more comprehensive tool than `focus-send`, with features that would break the
goal of being compatible with Kaleidoscope's `bin/focus-send`. The tool provides
a number of sub-commands, all of which share a common set of options (unless
stated otherwise):

#### Shared options

- `-d`, `--device` `<PATH>`: The device to connect to. If not specified, the
  tool will find all supported devices, and connect to the first found.
- `-c`, `--chunk-size` `<CHUNK_SIZE>`: Sets the chunk size to use when sending data. Defaults to 32, the same as Chrysalis. Setting the chunk size to zero will disable chunking, and all data will be written in one go.
- `-q`, `--quiet`: The tool displays a progress indicator by default. If this
  option is specified, it will not display one.

#### `help`

Prints a help screen.

Does not support the shared options, but can be given a sub-command, in which
case it will print the help screen of the sub-command, rather than the tools
own.

#### `list-ports`

Lists port paths that belong to auto-detected supported devices.

Does not support the shared options.

#### `send <COMMAND> [<ARGUMENTS...>]`

Send the given `<COMMAND>` to the device, wait for, and then display the reply.
The `<COMMAND>` is mandatory, `<ARGUMENTS...>` are optional.

#### `backup`

Reads every setting from the keyboard, and outputs a JSON-formatted backup to
standard output. The output can be fed back to the `restore` command.

#### `restore`

Reads a JSON-formatted backup from the standard input, and restores the settings
stores within it onto the keyboard.
