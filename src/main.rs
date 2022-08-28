// focus-send -- Bare-bones Focus testing tool
// Copyright (C) 2022  Keyboard.io, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use clap::Parser;
use serialport::SerialPort;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
const DEFAULT_DEVICE: &str = "/dev/ttyACM0";
#[cfg(target_os = "macos")]
const DEFAULT_DEVICE: &str = "/dev/cu.usbmodemCkbio01E";
#[cfg(target_family = "windows")]
const DEFAULT_DEVICE: &str = "COM1";

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(
        short = 'd',
        long = "device",
        env = "DEVICE",
        hide_env = true,
        value_name = "PATH",
        help = "The device to connect to",
        default_value = DEFAULT_DEVICE,
    )]
    device: String,

    command: String,
    args: Vec<String>,
}

fn main() {
    let opts = Cli::parse();

    let mut port = serialport::new(&opts.device, 11520)
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", opts.device, e);
            ::std::process::exit(1);
        });

    flush(&mut port);

    send_request(&mut port, opts.command, opts.args)
        .expect("failed to send the request to the keyboard");

    wait_for_data(&*port);

    let reply = read_reply(&mut port).expect("failed to read the reply");
    println!("{}", reply);
}

// Send an empty command, and consume any replies. This should clear any pending
// commands or output.
fn flush(port: &mut Box<dyn SerialPort>) {
    send_request(port, String::from(" "), vec![]).expect("failed to send an empty command");
    wait_for_data(&**port);
    read_reply(port).expect("failed to flush the device");
}

fn send_request(
    port: &mut Box<dyn SerialPort>,
    command: String,
    args: Vec<String>,
) -> Result<(), std::io::Error> {
    let mut request_parts = vec![command];
    request_parts.extend(args);
    let request = request_parts.join(" ") + "\n";

    port.write_all(request.as_bytes())
}

fn wait_for_data(port: &dyn SerialPort) {
    while port.bytes_to_read().expect("Error calling bytes_to_read") == 0 {
        thread::sleep(Duration::from_millis(100));
    }
}

fn read_reply(port: &mut Box<dyn SerialPort>) -> Result<String, std::io::Error> {
    let mut buffer: Vec<u8> = vec![0; 1024];
    let mut result: String = String::from("");
    loop {
        match port.read(buffer.as_mut_slice()) {
            Ok(t) => {
                result = result + &String::from_utf8_lossy(&buffer[..t]);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    Ok(cleanup_reply(result))
}

fn cleanup_reply(reply: String) -> String {
    let mut lines: Vec<&str> = reply.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if lines[i] == "." || lines[i].is_empty() {
            lines.remove(i);
        } else {
            i += 1;
        }
    }
    lines.join("\n")
}
