# IO Arg

Conviniently accept either standard streams or file paths in CLI tools written in Rust as arguments.

## Usage

```rust
use io_arg::IoArg;
use structopt::StructOpt;

/// A command line tool taking a required input argument and an optional output argument.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Path to input file. Set to "-" to use STDIN instead of a file.
    input: IoArg,
    /// Path to output file. Leave out or set to "-" to use STDOUT instead of a file.
    #[structopt(long, short = "o", default_value = "-")]
    output: IoArg,
}
```
