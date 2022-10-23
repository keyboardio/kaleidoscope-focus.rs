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

#![warn(missing_docs)]
#![allow(rustdoc::broken_intra_doc_links)]

//! **Talking to [`Kaleidoscope`] powered devices with Rust**
//!
//! This library is a very thin layer on top of `serialport`, implementing a
//! handful of convenience functions to make it easy to communicate with devices
//! speaking Kaleidoscope's [`Focus`] protocol.
//!
//! Start at [`struct.Focus`] to discover what the crate provides.
//!
//! [`struct.Focus`]: ./struct.Focus.html
//! [`Kaleidoscope`]: https://github.com/keyboardio/Kaleidoscope
//! [`Focus`]: https://kaleidoscope.readthedocs.io/en/latest/plugins/Kaleidoscope-FocusSerial.html

use serialport::SerialPort;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// The representation of a connection to a keyboard, used for all communication.
///
/// Constructed using a builder pattern, using [`Focus::create`].
pub struct Focus {
    port: Box<dyn SerialPort>,
    chunk_size: usize,
    interval: u64,
    progress_report: Box<dyn Fn(usize) + 'static>,
}

impl Focus {
    /// Create a new connection using a Builder pattern.
    ///
    /// A `device` to open must be specified. What the `device` is, is platform
    /// dependent, see [`serialport::new`] for more information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let mut conn = Focus::create("/dev/ttyACM0")
    ///     .chunk_size(32)
    ///     .interval(50)
    ///     .open()?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn create(device: &str) -> FocusBuilder {
        FocusBuilder {
            device,
            chunk_size: 32,
            interval: 50,
        }
    }

    /// Send a request to the keyboard.
    ///
    /// Sends a `command` request to the keyboard, with optional `args`. Returns
    /// the reply to the request.
    ///
    /// May return an empty string if the command is unknown, or if it does not
    /// have any output.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let mut conn = Focus::create("/dev/ttyACM0").open()?;
    /// let reply = conn.request("help", None);
    /// assert!(reply.is_ok());
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// # use indicatif::ProgressBar;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let progress = ProgressBar::new(0);
    /// let mut conn = Focus::create("/dev/ttyACM0").open()?;
    /// conn.set_progress_report(move |delta| {
    ///   progress.inc(delta.try_into().unwrap());
    /// });
    /// let reply = conn.request("settings.version", None)?;
    /// assert_eq!(reply, "1 ");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn request(
        &mut self,
        command: &str,
        args: Option<&[String]>,
    ) -> Result<String, std::io::Error> {
        self.send(command, args)?.receive()
    }

    fn send(
        &mut self,
        command: &str,
        args: Option<&[String]>,
    ) -> Result<&mut Self, std::io::Error> {
        let request = format!("{} {}\n", command, args.unwrap_or_default().join(" "));
        self.port.write_data_terminal_ready(true)?;

        if self.chunk_size > 0 {
            for c in request.as_bytes().chunks(self.chunk_size) {
                self.port.write_all(c)?;
                thread::sleep(Duration::from_millis(self.interval));
                (self.progress_report)(c.len());
            }
        } else {
            self.port.write_all(request.as_bytes())?;
            (self.progress_report)(request.len());
        }

        Ok(self)
    }

    fn receive(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = [0; 1024];
        let mut reply = vec![];

        self.port.read_data_set_ready()?;
        self.wait_for_data()?;

        loop {
            match self.port.read(buffer.as_mut_slice()) {
                // EOF
                Ok(0) => break,
                Ok(t) => {
                    reply.extend(&buffer[..t]);
                    (self.progress_report)(t);
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
            .lines()
            .filter(|l| !l.is_empty() && *l != ".")
            .collect::<Vec<&str>>()
            .join("\n"))
    }

    /// Send a command - a request without arguments - to the keyboard.
    ///
    /// See [`Focus::request`], this is the same, but without any arguments.
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let mut conn = Focus::create("/dev/ttyACM0").open()?;
    /// let reply = conn.command("settings.version")?;
    /// assert_eq!(reply, "1 ");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn command(&mut self, command: &str) -> Result<String, std::io::Error> {
        self.request(command, None)
    }

    /// Set the progress reporter function for I/O operations.
    ///
    /// Whenever I/O happens, the progress reporter function is called. This can
    /// be used to display progress bars and the like. The reporter function
    /// takes a single `usize` argument, and returns nothing.
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// # use indicatif::ProgressBar;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let progress = ProgressBar::new(0);
    /// let mut conn = Focus::create("/dev/ttyACM0").open()?;
    /// conn.set_progress_report(move |delta| {
    ///   progress.inc(delta.try_into().unwrap());
    /// });
    /// let reply = conn.command("version");
    /// assert!(reply.is_ok());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn set_progress_report(&mut self, progress_report: impl Fn(usize) + 'static) {
        self.progress_report = Box::new(progress_report);
    }

    /// Flush any pending data.
    ///
    /// Sends an empty command, and then waits until the keyboard stops sending
    /// data. The intended use is to clear any pending I/O operations in flight.
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let mut conn = Focus::create("/dev/ttyACM0").open()?;
    ///
    /// /// Send a request whose output we're not interested in.
    /// conn.command("help")?;
    /// /// Flush it!
    /// conn.flush()?;
    ///
    /// /// ...and then send the request we want the output of.
    /// let reply = conn.command("settings.version")?;
    /// assert_eq!(reply, "1 ");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn flush(&mut self) -> Result<&mut Self, std::io::Error> {
        self.command(" ")?;
        Ok(self)
    }

    /// Return the port name - if known - of the connected device.
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let mut conn = Focus::create("/dev/ttyACM0").open()?;
    /// assert_eq!(conn.port_name(), Some("/dev/ttyACM0".to_string()));
    /// #   Ok(())
    /// # }
    /// ```
    pub fn port_name(&self) -> Option<String> {
        self.port.name()
    }

    /// Find supported devices, and return the paths to their ports.
    ///
    /// Iterates over available USB serial ports, and keeps only those that belong
    /// to a supported keyboard. The crate only recognises Keyboardio devices as
    /// supported keyboards.
    ///
    /// ```no_run
    /// # use kaleidoscope_focus::Focus;
    /// let devices = Focus::find_devices().unwrap();
    /// assert!(devices.len() > 0);
    /// ```
    pub fn find_devices() -> Option<Vec<String>> {
        #[derive(PartialEq)]
        struct DeviceDescriptor {
            vid: u16,
            pid: u16,
        }
        impl From<&serialport::UsbPortInfo> for DeviceDescriptor {
            fn from(port: &serialport::UsbPortInfo) -> Self {
                Self {
                    vid: port.vid,
                    pid: port.pid,
                }
            }
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

        let devices: Vec<String> = serialport::available_ports()
            .ok()?
            .iter()
            .filter_map(|p| match &p.port_type {
                serialport::SerialPortType::UsbPort(port_info) => supported_keyboards
                    .contains(&port_info.into())
                    .then(|| p.port_name.to_string()),
                _ => None,
            })
            .collect();

        if devices.is_empty() {
            return None;
        }

        Some(devices)
    }

    fn wait_for_data(&mut self) -> Result<(), std::io::Error> {
        while self.port.bytes_to_read()? == 0 {
            thread::sleep(Duration::from_millis(self.interval));
        }
        Ok(())
    }
}

/// Provides a builder pattern for [`Focus`].
///
/// Use [`Focus::create`] to start building.
pub struct FocusBuilder<'a> {
    device: &'a str,
    chunk_size: usize,
    interval: u64,
}

impl FocusBuilder<'_> {
    /// Set the chunk size to use for writes.
    ///
    /// The library uses chunked writes by default, to work around old firmware
    /// bugs, and operating system quirks at times. Use this method to set the
    /// chunk size to your desired value.
    ///
    /// Setting the size to 0 disables chunking.
    ///
    /// See [`Focus::create`] for an example.
    pub fn chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    /// Set the interval between chunks.
    ///
    /// See [`Focus::create`] for an example.
    pub fn interval(mut self, interval: u64) -> Self {
        self.interval = interval;
        self
    }

    /// Open a connection to the keyboard.
    ///
    /// Stops building the configuration for the [`Focus`] struct, and opens a
    /// connection to the keyboard.
    ///
    /// See [`Focus::create`] for an example.
    pub fn open(&self) -> Result<Focus, serialport::Error> {
        let port = serialport::new(self.device, 115200)
            .timeout(Duration::from_millis(self.interval))
            .open()?;

        Ok(Focus {
            port,
            chunk_size: self.chunk_size,
            interval: self.interval,
            progress_report: Box::new(|_| {}),
        })
    }
}
