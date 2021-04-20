use std::{
    fs::File,
    io::{self, stdout, Stdout, Write},
};

use crate::IoArg;

/// An opend outup stream which can provide a `io::Write` interface.
pub enum Output {
    StdOut(Stdout),
    File(File),
}

impl Output {
    /// Either calls `stdout` or `File::create` depending on `io_arg`.
    pub fn new(io_arg: IoArg) -> io::Result<Self> {
        let ret = match io_arg {
            IoArg::StdStream => Output::StdOut(stdout()),
            IoArg::File(path) => Output::File(File::create(path)?),
        };
        Ok(ret)
    }

    /// Wraps either standard out or the file in a `Box<dyn Write>`. The resulting writer mutably
    /// borrows the instance, since it may lock standard out.
    pub fn write(&mut self) -> Box<dyn Write + '_> {
        match self {
            Output::StdOut(stream) => Box::new(stream.lock()),
            Output::File(file) => Box::new(file),
        }
    }
}
