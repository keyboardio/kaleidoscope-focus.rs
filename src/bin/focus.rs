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
use indicatif::{ProgressBar, ProgressStyle};
use kaleidoscope_focus::Focus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

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

#[derive(Args)]
struct Send {
    /// The command to send
    command: String,
    /// Optional arguments for <COMMAND>
    args: Vec<String>,
}

fn main() {
    let opts = Cli::parse();

    match &opts.command {
        Commands::ListPorts => list_ports(),
        Commands::Send(s) => send(s, &opts),
        Commands::Backup => backup(&opts),
        Commands::Restore => restore(&opts),
    }
}

fn list_ports() {
    kaleidoscope_focus::find_devices()
        .expect("No supported devices found")
        .iter()
        .for_each(|device| {
            println!("{}", device);
        });
}

fn connect(opts: &Cli) -> Focus {
    let device_path = match &opts.device {
        Some(d) => d.to_string(),
        None => kaleidoscope_focus::find_devices().expect("No supported device found")[0].clone(),
    };

    Focus::create(&device_path)
        .chunk_size(opts.chunk_size)
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", &device_path, e);
            ::std::process::exit(1);
        })
}

fn send(opts: &Send, main_opts: &Cli) {
    let mut focus = connect(main_opts);

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

#[derive(Serialize, Deserialize, Debug)]
struct Backup {
    commands: HashMap<String, String>,
}

fn backup(main_opts: &Cli) {
    let mut focus = connect(main_opts);

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

fn restore(main_opts: &Cli) {
    let backup: Backup = serde_json::from_reader(io::stdin()).expect("Unable to parse the backup");

    let mut focus = connect(main_opts);
    let pb = if main_opts.quiet {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(backup.commands.len().try_into().unwrap())
    };

    pb.set_style(ProgressStyle::with_template("{spinner} {pos} / {len} ({msg}) ").unwrap());
    backup.commands.iter().for_each(|(k, v)| {
        pb.set_message(k.clone());
        focus
            .request(k.to_string(), Some(vec![v.to_string()]), None)
            .expect("Restoration failed");
        focus.read_reply().expect("Restoration failed");
        pb.inc(1);
    });
    pb.finish_and_clear();
}
