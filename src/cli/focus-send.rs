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

mod shared;
use crate::shared::{Cli, ConnectionOptions};

#[derive(Parser)]
#[command(version, about)]
struct Options {
    #[arg(short, long, env, hide_env = true, value_name = "PATH")]
    /// The device to connect to
    device: Option<String>,

    /// The command to send
    command: String,
    /// Optional arguments for <COMMAND>
    args: Vec<String>,
}

fn main() {
    let opts = Options::parse();

    let mut cli = Cli::connect(ConnectionOptions {
        device: opts.device,
        chunk_size: 32,
        quiet: true,
    });

    cli.send(&opts.command, &opts.args)
        .expect("Error communicating with the keyboard");
}
