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
use indicatif::{ProgressBar, ProgressStyle};
use std::io;

use crate::commands::backup::BackupData;
use crate::commands::{connect, ConnectionOptions};

#[derive(Args)]
pub struct Restore {
    #[command(flatten)]
    pub shared: ConnectionOptions,
}

#[allow(dead_code)]
pub fn restore(opts: &Restore) {
    let backup: BackupData =
        serde_json::from_reader(io::stdin()).expect("Unable to parse the backup");

    let mut focus = connect(&opts.shared);
    let pb = if opts.shared.quiet {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(backup.commands.len().try_into().unwrap())
    };

    pb.set_style(ProgressStyle::with_template("{spinner} {pos} / {len} ({msg}) ").unwrap());
    backup.restore.iter().for_each(|k| {
        pb.set_message(k.clone());
        if let Some(v) = backup.commands.get(k) {
            focus
                .request(k.to_string(), Some(vec![v.to_string()]), None)
                .expect("Restoration failed");
            focus.read_reply().expect("Restoration failed");
        }
        pb.inc(1);
    });
    pb.finish_and_clear();
}
