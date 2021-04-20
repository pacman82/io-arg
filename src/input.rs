use std::{
    fs::File,
    io::{self, stdin, BufRead, BufReader, Stdin},
};

use crate::IoArg;

/// An opend input stream which can provide a `io::Read` interface.
pub enum Input {
    StdIn(Stdin),
    File(File),
}

impl Input {
    /// Either calls `stdin` or `File::open` depending on `io_arg`.
    pub fn new(io_arg: IoArg) -> io::Result<Self> {
        let ret = match io_arg {
            IoArg::StdStream => Input::StdIn(stdin()),
            IoArg::File(path) => Input::File(File::open(path)?),
        };
        Ok(ret)
    }

    /// Either locks stdin or wraps the file in a `BufReader`.
    pub fn buf_read(&mut self) -> Box<dyn BufRead + '_> {
        match self {
            Input::StdIn(stream) => Box::new(stream.lock()),
            Input::File(file) => Box::new(BufReader::new(file)),
        }
    }
}
