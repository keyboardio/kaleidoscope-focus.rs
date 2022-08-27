// focus-send -- Bare-bones Focus testing tool
// Copyright (C) 2022  Keyboard.io, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use clap::Parser;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short = 'd', value_name = "PATH", default_value = "/dev/ttyACM0")]
    device: String,

    command: String,
    args: Vec<String>
}

fn main() {
    let opts = Cli::parse();

    let mut port = serialport::new(&opts.device, 11520)
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", opts.device, e);
            ::std::process::exit(1);
        });

    // Write the request
    let mut request_parts = vec![opts.command];
    request_parts.extend(opts.args);
    let request = request_parts.join(" ") + "\n";

    port.write_all(request.as_bytes()).unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        ::std::process::exit(1);
    });

    // Wait until we have something to read
    while port.bytes_to_read().expect("Error calling bytes_to_read") == 0 {
        thread::sleep(Duration::from_millis(100));
    }

    // Read reply
    let mut buffer: Vec<u8> = vec![0; 1024];
    loop {
        match port.read(buffer.as_mut_slice()) {
            Ok(t) => {
                io::stdout().write_all(&buffer[..t]).unwrap();
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                break;
            },
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
