use std::{fs::File, io::{self, BufWriter, Stdout, Write, stdout}};

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

    /// Wraps either standard out or the file in a `Box<dyn Write>`. In case of [`Output::StdOut`]
    /// standard out will be locked. In case of [`Output::File`] the file will be wrapped in
    /// [`std::io::BufWriter`] in order to minimize system calls.
    pub fn into_write(self) -> Box<dyn Write> {
        match self {
            Output::StdOut(stream) => Box::new(stream.lock()),
            Output::File(file) => Box::new(BufWriter::new(file)),
        }
    }
}
