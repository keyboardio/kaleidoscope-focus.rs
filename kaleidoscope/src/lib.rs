// kaleidoscope -- Talk with Kaleidoscope powered devices
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

use serialport::SerialPort;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct Focus {
    port: Box<dyn SerialPort>,
    chunk_size: usize,
    interval: u64,
}

pub struct FocusBuilder<'a> {
    device: &'a str,
    chunk_size: usize,
    interval: u64,
}

impl FocusBuilder<'_> {
    pub fn open(&self) -> Result<Focus, serialport::Error> {
        let port = serialport::new(self.device, 11520)
            .timeout(Duration::from_millis(self.interval))
            .open()?;

        Ok(Focus {
            port,
            chunk_size: self.chunk_size,
            interval: self.interval,
        })
    }

    pub fn chunk_size(&mut self, chunk_size: usize) -> &Self {
        self.chunk_size = chunk_size;
        self
    }

    pub fn interval(&mut self, interval: u64) -> &Self {
        self.interval = interval;
        self
    }
}

impl Focus {
    pub fn create(device: &str) -> FocusBuilder {
        FocusBuilder {
            device,
            chunk_size: 32,
            interval: 50,
        }
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.request(String::from(" "), None)?;
        self.read_reply()?;
        Ok(())
    }

    pub fn request(
        &mut self,
        command: String,
        args: Option<Vec<String>>,
    ) -> Result<(), std::io::Error> {
        self.request_with_progress(command, args, |_| {}, |_| {})
    }

    pub fn request_with_progress<FL, FP>(
        &mut self,
        command: String,
        args: Option<Vec<String>>,
        set_length: FL,
        progress: FP,
    ) -> Result<(), std::io::Error>
    where
        FL: Fn(usize),
        FP: Fn(usize),
    {
        let request = [vec![command], args.unwrap_or_default()].concat().join(" ") + "\n";
        self.port.write_data_terminal_ready(true)?;

        set_length(request.len());

        for c in request.as_bytes().chunks(self.chunk_size) {
            progress(c.len());
            self.port.write_all(c)?;
            thread::sleep(Duration::from_millis(self.interval));
        }

        Ok(())
    }

    pub fn read_reply(&mut self) -> Result<String, std::io::Error> {
        let mut buffer: Vec<u8> = vec![0; 1024];
        let mut reply = vec![];

        self.port.read_data_set_ready()?;
        self.wait_for_data()?;

        loop {
            match self.port.read(buffer.as_mut_slice()) {
                Ok(t) => {
                    reply.extend(&buffer[..t]);
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }

            thread::sleep(Duration::from_millis(self.interval));
        }

        Ok(String::from_utf8_lossy(&reply)
            .to_string()
            .lines()
            .filter(|l| !l.is_empty() && *l != ".")
            .collect::<Vec<&str>>()
            .join("\n"))
    }

    fn wait_for_data(&mut self) -> Result<(), std::io::Error> {
        while self.port.bytes_to_read()? == 0 {
            thread::sleep(Duration::from_millis(self.interval));
        }
        Ok(())
    }
}

pub fn find_devices() -> Option<Vec<String>> {
    #[derive(PartialEq)]
    struct DeviceDescriptor {
        vid: u16,
        pid: u16,
    }
    let supported_keyboards = [
        // Keyboardio Model100
        DeviceDescriptor {
            vid: 0x3496,
            pid: 0x0006,
        },
        // Keyboardio Atreus
        DeviceDescriptor {
            vid: 0x1209,
            pid: 0x2303,
        },
        // Keyboardio Model01
        DeviceDescriptor {
            vid: 0x1209,
            pid: 0x2301,
        },
    ];

    // Otherwise list the serial ports, and return the first USB serial port
    // that has a vid/pid that matches any of the Keyboardio devices.
    Some(
        serialport::available_ports()
            .ok()?
            .iter()
            .filter_map(|p| match &p.port_type {
                serialport::SerialPortType::UsbPort(port_info) => {
                    struct MinimalPortInfo {
                        ids: DeviceDescriptor,
                        port: String,
                    }
                    Some(MinimalPortInfo {
                        ids: DeviceDescriptor {
                            vid: port_info.vid,
                            pid: port_info.pid,
                        },
                        port: p.port_name.to_string(),
                    })
                }
                _ => None,
            })
            .filter_map(|p| supported_keyboards.contains(&p.ids).then(|| p.port))
            .collect(),
    )
}
