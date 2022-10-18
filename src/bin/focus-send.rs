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

mod commands;
use crate::commands::{send::Send, ConnectionOptions};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, env, hide_env = true, value_name = "PATH")]
    /// The device to connect to
    device: Option<String>,

    /// The command to send
    command: String,
    /// Optional arguments for <COMMAND>
    args: Vec<String>,
}

fn main() {
    let opts = Cli::parse();
    let send_opts = Send {
        shared: ConnectionOptions {
            device: opts.device,
            chunk_size: 32,
            quiet: true,
        },
        command: opts.command.to_string(),
        args: opts.args.clone(),
    };

    commands::send(&send_opts);
}
