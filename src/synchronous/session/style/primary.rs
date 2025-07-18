// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![cfg(all(feature = "sync", not(any(feature = "tokio", feature = "smol"))))]

use crate::{
    options::SessionOptions,
    style::{private, SessionStyle},
};

use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

/// Primary session.
pub struct Primary {
    /// TCP stream used to communicate with router.
    stream: BufReader<TcpStream>,

    /// Session options.
    _options: SessionOptions,
}

impl private::SessionStyle for Primary {
    fn new(_options: SessionOptions) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            stream: BufReader::new(TcpStream::connect(format!(
                "127.0.0.1:{}",
                _options.samv3_tcp_port
            ))?),
            _options,
        })
    }

    fn write_command(&mut self, command: &[u8]) -> crate::Result<()> {
        self.stream.get_mut().write_all(command).map_err(From::from)
    }

    fn read_command(&mut self) -> crate::Result<String> {
        let mut response = String::new();

        self.stream.read_line(&mut response).map(|_| response).map_err(From::from)
    }

    fn create_session(&self) -> private::SessionParameters {
        private::SessionParameters {
            style: "PRIMARY".to_string(),
            options: Vec::new(),
        }
    }
}

impl SessionStyle for Primary {}
