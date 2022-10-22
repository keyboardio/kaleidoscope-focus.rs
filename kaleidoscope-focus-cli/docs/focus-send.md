# focus-send

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
