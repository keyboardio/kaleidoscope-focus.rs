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
use indicatif::{ProgressBar, ProgressStyle};
use kaleidoscope_focus::Focus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

#[derive(Args)]
pub struct ConnectionOptions {
    #[arg(short, long, env, hide_env = true, value_name = "PATH")]
    /// The device to connect to
    pub device: Option<String>,

    #[arg(short, long, default_value = "32")]
    /// Set the size of the buffer used to send data. Setting it to 0 writes
    /// everything all at once
    pub chunk_size: usize,

    #[arg(short, long, default_value = "false")]
    /// Operate quietly
    pub quiet: bool,
}

pub struct Cli {
    conn: Focus,
    progress: ProgressBar,
}

#[allow(dead_code)]
impl Cli {
    pub fn connect(opts: ConnectionOptions) -> Self {
        let device_path = match &opts.device {
            Some(d) => d.to_string(),
            None => Focus::find_devices()
                .expect("No supported device found")
                .remove(0),
        };

        let mut conn = Focus::create(&device_path)
            .chunk_size(opts.chunk_size)
            .open()
            .unwrap_or_else(|e| {
                eprintln!("Failed to open \"{}\". Error: {}", &device_path, e);
                ::std::process::exit(1);
            });
        let progress = if opts.quiet {
            ProgressBar::hidden()
        } else {
            ProgressBar::new(0)
        };
        progress.set_style(ProgressStyle::with_template("{spinner} {prefix}{msg}").unwrap());

        let cloned_progress = progress.clone();
        conn.set_progress_report(move |delta| {
            cloned_progress.inc(delta.try_into().unwrap());
        });

        Self { conn, progress }
    }

    pub fn send(&mut self, command: &str, args: &[String]) -> Result<()> {
        let reply = self.conn.flush()?.request(command, Some(args))?;
        self.progress.finish_and_clear();

        if !reply.is_empty() {
            println!("{}", reply);
        }

        Ok(())
    }

    pub fn list_ports() -> Result<()> {
        Focus::find_devices()
            .expect("No supported devices found")
            .iter()
            .for_each(|device| {
                println!("{}", device);
            });
        Ok(())
    }

    pub fn backup(&mut self) -> Result<()> {
        self.progress.set_prefix("backing up: ");

        let reply = self.conn.flush()?.command("backup")?;

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
        for cmd in &backup_commands {
            self.progress.set_message(cmd.to_string());
            let reply = self.conn.command(cmd)?;
            if !reply.is_empty() {
                backup.commands.insert(cmd.to_string(), reply);
            } else {
                backup.restore.retain(|x| x != cmd);
            }
            self.progress.inc(1);
        }
        self.progress.finish_and_clear();

        println!("{}", serde_json::to_string(&backup)?);
        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        let backup: BackupData =
            serde_json::from_reader(io::stdin()).expect("Unable to parse the backup");

        self.progress.set_prefix("restoring: ");

        for k in &backup.restore {
            self.progress.set_message(k.clone());
            if let Some(v) = backup.commands.get(k) {
                self.conn.request(k, Some(&[v.to_string()]))?;
            }
            self.progress.inc(1);
        }
        self.progress.finish_and_clear();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BackupData {
    pub restore: Vec<String>,
    pub commands: HashMap<String, String>,
}
