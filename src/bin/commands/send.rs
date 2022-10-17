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
use indicatif::ProgressBar;

use crate::commands::{connect, MainOptions};

#[derive(Args)]
pub struct Send {
    /// The command to send
    pub command: String,
    /// Optional arguments for <COMMAND>
    pub args: Vec<String>,
}

pub fn send(opts: &Send, main_opts: MainOptions) {
    let mut focus = connect(&main_opts);

    let pb = if !opts.args.is_empty() && !main_opts.quiet {
        ProgressBar::new(100)
    } else {
        ProgressBar::hidden()
    };
    focus.flush().unwrap();
    focus
        .request(opts.command.to_string(), Some(opts.args.clone()), Some(&pb))
        .expect("failed to send the request to the keyboard");
    pb.finish_and_clear();

    let reply = focus.read_reply().expect("failed to read the reply");
    if !reply.is_empty() {
        println!("{}", reply);
    }
}
