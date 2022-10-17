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

use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::commands::{connect, MainOptions};

#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    pub commands: HashMap<String, String>,
}

#[allow(dead_code)]
pub fn backup(main_opts: MainOptions) {
    let mut focus = connect(&main_opts);

    focus.flush().unwrap();
    focus
        .request("backup".to_string(), None, None)
        .expect("Failed to request backup eligible commands");
    let reply = focus
        .read_reply()
        .expect("failed to read the list of backup eligible commands");

    let backup_commands: Vec<&str> = reply.lines().collect();
    if backup_commands.is_empty() {
        eprintln!("The `backup` command appears to be unsupported by the firmware.");
        ::std::process::exit(1);
    }

    let mut backup = Backup {
        commands: HashMap::new(),
    };
    let pb = if main_opts.quiet {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(backup_commands.len().try_into().unwrap())
    };
    backup_commands.iter().for_each(|cmd| {
        focus
            .request(cmd.to_string(), None, None)
            .expect("Failed to send command");
        let reply = focus.read_reply().expect("Failed to read a reply");
        backup.commands.insert(cmd.to_string(), reply);
        pb.inc(1);
    });
    pb.finish_and_clear();

    println!("{}", serde_json::to_string(&backup).unwrap());
}
