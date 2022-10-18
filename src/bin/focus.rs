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
use crate::commands::{backup::Backup, restore::Restore, send::Send};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
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
    Backup(Backup),
    /// Restore the keyboards configuration from backup
    Restore(Restore),
}

fn main() {
    let opts = Cli::parse();

    match &opts.command {
        Commands::ListPorts => commands::list_ports(),
        Commands::Send(s) => commands::send(s),
        Commands::Backup(b) => commands::backup(b),
        Commands::Restore(r) => commands::restore(r),
    }
}
