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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::commands::{connect, ConnectionOptions};

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupData {
    pub restore: Vec<String>,
    pub commands: HashMap<String, String>,
}

#[derive(Args)]
pub struct Backup {
    #[command(flatten)]
    pub shared: ConnectionOptions,
}

#[allow(dead_code)]
pub fn backup(opts: &Backup) {
    let mut focus = connect(&opts.shared);
    let progress = if opts.shared.quiet {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(0)
            .with_style(ProgressStyle::with_template("{spinner} backing up: {msg}").unwrap())
    };

    let reply = focus
        .flush()
        .unwrap()
        .request("backup", None, Some(&progress))
        .expect("Failed to request backup eligible commands")
        .read_reply(Some(&progress))
        .expect("failed to read the list of backup eligible commands");

    let mut backup_commands: Vec<&str> = reply.lines().collect();
    if backup_commands.is_empty() {
        // If the `backup` command is not supported, fall back to a static list
        // of commands. This is a lost of commands in Kaleidoscope proper that
        // can be backed up as of 2022-10-17.
        //
        // The static list is here to make it seamless to backup older firmware
        // too.
        backup_commands = vec![
            "autoshift.categories",
            "autoshift.timeout",
            "colormap.map",
            "escape_oneshot.cancel_key",
            "hardware.keyscan",
            "hardware.side_power",
            "hardware.sled_current",
            "hostos.type",
            "idleleds.time_limit",
            "keymap.custom",
            "keymap.layerNames",
            "keymap.onlyCustom",
            "led.brightness",
            "led_mode.default",
            "macros.map",
            "mousekeys.accel_duration",
            "mousekeys.base_speed",
            "mousekeys.init_speed",
            "mousekeys.scroll_interval",
            "oneshot.auto_layers",
            "oneshot.auto_mods",
            "oneshot.double_tap_timeout",
            "oneshot.hold_timeout",
            "oneshot.stickable_keys",
            "oneshot.timeout",
            "palette",
            "settings.defaultLayer",
            "spacecadet.mode",
            "spacecadet.timeout",
            "tapdance.map",
            "typingbreaks.idleTimeLimit",
            "typingbreaks.leftMaxKeys",
            "typingbreaks.lockLength",
            "typingbreaks.lockTimeOut",
            "typingbreaks.rightMaxKeys",
        ];
    }

    let mut backup = BackupData {
        commands: HashMap::new(),
        restore: backup_commands.iter().map(|cmd| cmd.to_string()).collect(),
    };
    backup_commands.iter().for_each(|cmd| {
        progress.set_message(cmd.to_string());
        let reply = focus
            .request(cmd, None, Some(&progress))
            .expect("Failed to send command")
            .read_reply(Some(&progress))
            .expect("Failed to read a reply");
        if !reply.is_empty() {
            backup.commands.insert(cmd.to_string(), reply);
        } else {
            backup.restore.retain(|x| x != cmd);
        }
        progress.inc(1);
    });
    progress.finish_and_clear();

    println!("{}", serde_json::to_string(&backup).unwrap());
}
