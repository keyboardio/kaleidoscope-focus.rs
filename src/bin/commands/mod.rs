// focus -- focus interaction tool
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

use clap::Args;
use kaleidoscope_focus::Focus;

pub mod backup;
pub mod list_ports;
pub mod restore;
pub mod send;

pub use backup::backup;
pub use list_ports::list_ports;
pub use restore::restore;
pub use send::send;

#[derive(Args)]
pub struct ConnectionOptions {
    #[arg(short, long, env, hide_env = true, value_name = "PATH")]
    /// The device to connect to
    pub device: Option<String>,

    #[arg(short, long, default_value = "32")]
    /// Set the size of the buffer used to send data. Setting it to 0 writes
    /// everything all at once
    pub chunk_size: usize,

    #[arg(short, long, default_value = "false")]
    /// Operate quietly
    pub quiet: bool,
}

fn connect(opts: &ConnectionOptions) -> Focus {
    let device_path = match &opts.device {
        Some(d) => d.to_string(),
        None => kaleidoscope_focus::find_devices().expect("No supported device found").remove(0)
    };

    Focus::create(&device_path)
        .chunk_size(opts.chunk_size)
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", &device_path, e);
            ::std::process::exit(1);
        })
}
