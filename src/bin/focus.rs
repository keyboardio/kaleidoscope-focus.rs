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
use indicatif::ProgressBar;
use kaleidoscope_focus::Focus;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, env, hide_env = true, value_name = "PATH")]
    /// The device to connect to
    device: Option<String>,

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
}

#[derive(Args)]
struct Send {
    #[arg(short, long, default_value = "32")]
    /// Set the size of the buffer used to send data. Setting it to 0 writes
    /// everything all at once
    chunk_size: usize,
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

fn send(opts: &Send, main_opts: &Cli) {
    let device_path = match &main_opts.device {
        Some(d) => d.to_string(),
        None => kaleidoscope_focus::find_devices().expect("No supported device found")[0].clone(),
    };

    let mut focus = Focus::create(&device_path)
        .chunk_size(opts.chunk_size)
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", &device_path, e);
            ::std::process::exit(1);
        });

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
