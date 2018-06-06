// Copyright 2018 The Open Cisterna project developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate serialport;

use maxsonar::serialport::prelude::*;
use std::io;
use std::str;

pub fn read_distance(port_name: &str) -> Result<u16, String>  {
    let settings: SerialPortSettings = Default::default();
    if let Ok(mut port) = serialport::open_with_settings(&port_name, &settings) {
        let mut serial_buf: Vec<u8> = vec![0; 5];
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) if t < 5 => (),
                Ok(_) => {
                    return match str::from_utf8(&serial_buf[..5]) {
                        Ok(v) => {
                            let stripped: String = v.chars().skip(1).collect();
                            stripped.parse::<u16>().or_else(|e| Err(e.to_string()))
                        },
                        Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e))
                    };
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    } else {
        Err(format!("Error: Port '{}' not available", &port_name))
    }
}
