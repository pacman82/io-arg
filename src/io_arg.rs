use std::{io, path::PathBuf, str::FromStr};

use crate::{Input, Output};

/// Argument for CLI tools which can either take a file or STDIN/STDOUT.
///
/// Caveat: stdin is represented as "-" at the command line. Which means your tool is going to have
/// a hard time operating on a file named "-".
///
/// ```
/// use clap::Parser;
/// use io_arg::IoArg;
///
/// #[derive(Debug, Parser)]
/// struct Cli {
///     /// Path to input file. Set to "-" to use STDIN instead of a file.
///     input: IoArg,
///     /// Path to output file. Leave out or set to "-" to use STDOUT instead of a file.
///     output: IoArg,
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IoArg {
    /// Indicates that the IO is connected to stdin/stdout. Represented as a "-" on the command line.
    StdStream,
    /// Indicates that the IO is connected to a file. Contains the file path. Just enter a path
    /// at the command line.
    File(PathBuf),
}

impl IoArg {
    /// Intended for use in an `if` expression or other situations there a boolean might be more
    /// convinient than matching against variants.
    ///
    /// # Return
    ///
    /// `true` if variant is `File`.
    /// `false` if variant is `StdStream`.
    pub fn is_file(&self) -> bool {
        match self {
            IoArg::StdStream => false,
            IoArg::File(_) => true,
        }
    }

    /// Either calls `stdin` or `File::open` depending on `io_arg`.
    pub fn open_as_input(self) -> io::Result<Input> {
        Input::new(self)
    }

    /// Either calls `stdout` or `File::create` depending on `io_arg`.
    pub fn open_as_output(self) -> io::Result<Output> {
        Output::new(self)
    }
}

impl FromStr for IoArg {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "-" => IoArg::StdStream,
            other => IoArg::File(other.into()),
        };
        Ok(out)
    }
}

#[cfg(test)]
mod tests {

    use super::IoArg;

    #[test]
    fn parsing() {
        let actual: IoArg = "-".parse().unwrap();
        assert_eq!(IoArg::StdStream, actual);

        let actual: IoArg = "filename".parse().unwrap();
        assert!(matches!(actual, IoArg::File(_)));
    }
}
