// focus-send -- Bare-bones Focus testing tool
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

use clap::Parser;
use indicatif::ProgressBar;
use kaleidoscope::Focus;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(
        short,
        long,
        env,
        hide_env = true,
        value_name = "PATH",
        help = "The device to connect to"
    )]
    device: Option<String>,
    #[arg(short, long, help = "Operate quietly", default_value = "false")]
    quiet: bool,

    command: String,
    args: Vec<String>,
}

fn main() {
    let opts = Cli::parse();
    let device = opts.device().unwrap_or_else(|| {
        eprintln!("No device found to connect to");
        ::std::process::exit(1);
    });

    let mut focus = Focus::create(&device).open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", &device, e);
        ::std::process::exit(1);
    });

    let pb = if !opts.args.is_empty() {
        ProgressBar::new(100)
    } else {
        ProgressBar::hidden()
    };
    focus.flush().unwrap();
    focus
        .request_with_progress(
            opts.command,
            Some(opts.args),
            |l| {
                pb.set_length(l.try_into().unwrap());
            },
            |c| {
                pb.inc(c.try_into().unwrap());
            },
        )
        .expect("failed to send the request to the keyboard");
    pb.finish_and_clear();
    let reply = focus.read_reply().expect("failed to read the reply");
    println!("{}", reply);
}

impl Cli {
    fn device(&self) -> Option<String> {
        // If we had a device explicitly specified, use that.
        if let Some(device) = &self.device {
            return Some(device.to_string());
        }

        let devices = kaleidoscope::find_devices()?;
        if devices.is_empty() {
            None
        } else {
            Some(devices[0].clone())
        }
    }
}
