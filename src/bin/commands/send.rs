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

use anyhow::Result;
use clap::Args;

use crate::commands::{connect, set_progress, ConnectionOptions};

#[derive(Args)]
pub struct Send {
    #[command(flatten)]
    pub shared: ConnectionOptions,

    /// The command to send
    pub command: String,
    /// Optional arguments for <COMMAND>
    pub args: Vec<String>,
}

pub fn send(opts: &Send) -> Result<()> {
    let mut focus = connect(&opts.shared);
    let progress = set_progress(&mut focus, &opts.shared);

    let reply = focus.flush()?.request(&opts.command, Some(&opts.args))?;
    progress.finish_and_clear();

    if !reply.is_empty() {
        println!("{}", reply);
    }

    Ok(())
}
