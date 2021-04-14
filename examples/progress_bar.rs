use std::{fs::File, io};

use indicatif::ProgressBar;
use io_arg::IoArg;
use structopt::StructOpt;
use atty::{isnt, Stream};

/// A command line tool taking a required input argument and an optional output argument.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Path to input file. Set to "-" to use STDIN instead of a file.
    input: IoArg,
    /// Path to output file. Leave out or set to "-" to use STDOUT instead of a file.
    #[structopt(long, short = "o", default_value = "-")]
    output: IoArg,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    // Only initialized in case `input` specifies a file path, because only then we have information
    // about input length.
    //
    // We keep this in top level scope, since we want the progress bar to live during the whole
    // program execution, so it will be displayed.
    let progress_bar = if args.input.is_file() && (args.output.is_file() || isnt(Stream::Stdout)) {
        let progress_bar = ProgressBar::new(0);
        Some(progress_bar)
    } else {
        None
    };

    // Keep our reference to stdin alive, if need be. Only initialized if we don't read from a file
    // and read from stdin. We hold it alive at top level scop, so we can hold the lock to it, for
    // duration of the program.
    let std_in;

    // Same story for `std_out` as for stdin. We keep it alive for the duration of the program, but
    // delay initiaization until we know we need it (i.e. we are writing to stdout and not into a
    // file, we open in this code).
    let std_out;

    let input: Box<dyn io::Read> = match args.input {
        IoArg::File(input) => {
            // Path argument specified. Open file and initialize progress bar.
            let file = File::open(&input)?;
            // Only show Progress bar, if input is a file and output is not /dev/tty.
            //
            // * We need the input to so we have the file metadata and therefore file length, to know
            // the amount of data we are going to proccess. Otherwise we can't set the length of the
            // progress bar.
            // * We don't want the Progress bar to interfere with the output, if writing to /dev/tty.
            // Progress bar interferes with formatting if stdout and stderr both go to /dev/tty
            if let Some(progress_bar) = &progress_bar {
                let len = file.metadata()?.len();
                progress_bar.set_length(len);
                let file_with_pbar = progress_bar.wrap_read(file);
                Box::new(file_with_pbar)
            } else {
                // Input file, but writing output to /dev/tty
                Box::new(file)
            }
        }
        IoArg::StdStream => {
            // Input path not set => Just use stdin
            std_in = io::stdin();
            Box::new(std_in.lock())
        }
    };

    let output: Box<dyn io::Write> = match args.output {
        IoArg::File(output) => {
            let writer = io::BufWriter::new(File::create(&output)?);
            Box::new(writer)
        }
        IoArg::StdStream => {
            std_out = io::stdout();
            let writer = io::BufWriter::new(std_out.lock());
            Box::new(writer)
        }
    };

    // Here the program would actually do stuff with `input` and `output`.

    let _ = input;
    let _ = output;

    Ok(())
}
