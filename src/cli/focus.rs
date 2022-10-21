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

use clap::{Args, Parser, Subcommand};

mod shared;
use crate::shared::{Cli, ConnectionOptions};

#[derive(Parser)]
#[command(version, about)]
struct Options {
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
    Backup(ConnectionOptions),
    /// Restore the keyboards configuration from backup
    Restore(ConnectionOptions),
}

#[derive(Args)]
pub struct Send {
    #[command(flatten)]
    pub shared: ConnectionOptions,

    /// The command to send
    pub command: String,
    /// Optional arguments for <COMMAND>
    pub args: Vec<String>,
}

fn main() {
    let opts = Options::parse();

    match opts.command {
        Commands::ListPorts => Cli::list_ports(),
        Commands::Send(s) => Cli::connect(s.shared).send(&s.command, &s.args),
        Commands::Backup(o) => Cli::connect(o).backup(),
        Commands::Restore(o) => Cli::connect(o).restore(),
    }
    .expect("Error communicating with the keyboard");
}
