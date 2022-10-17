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

use clap::{Parser, Subcommand};

mod commands;
use crate::commands::{send::Send, MainOptions};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, env, hide_env = true, value_name = "PATH")]
    /// The device to connect to
    device: Option<String>,

    #[arg(short, long, default_value = "32")]
    /// Set the size of the buffer used to send data. Setting it to 0 writes
    /// everything all at once
    chunk_size: usize,

    #[arg(short, long, default_value = "false")]
    /// Operate quietly
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(version, about)]
enum Commands {
    /// List available ports for focus-capable devices
    ListPorts,
    /// Send a request to the keyboard, and display the reply
    Send(Send),
    /// Create a backup of the keyboards configuration
    Backup,
    /// Restore the keyboards configuration from backup
    Restore,
}

impl From<&Cli> for MainOptions {
    fn from(opts: &Cli) -> Self {
        Self {
            device: opts.device.clone(),
            chunk_size: opts.chunk_size,
            quiet: opts.quiet,
        }
    }
}

fn main() {
    let opts = Cli::parse();

    match &opts.command {
        Commands::ListPorts => commands::list_ports(),
        Commands::Send(s) => commands::send(s, (&opts).into()),
        Commands::Backup => commands::backup((&opts).into()),
        Commands::Restore => commands::restore((&opts).into()),
    }
}
