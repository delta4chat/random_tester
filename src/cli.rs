//! the `entest` command line tools.

#![allow(warnings, missing_docs)]

/*
ent --  Calculate entropy of file.  Call
        with ent [options] [input-file]

        Options:   -b   Treat input as a stream of bits
                   -c   Print occurrence counts
                   -f   Fold upper to lower case letters
                   -t   Terse output in CSV format
                   -u   Print this message
*/

/*
Entropy = 7.980627 bits per byte.

Optimum compression would reduce the size
of this 51768 byte file by 0 percent.

Chi square distribution for 51768 samples is 1542.26, and randomly
would exceed this value less than 0.01 percent of the times.

Arithmetic mean value of data bytes is 125.93 (127.5 = random).
Monte Carlo value for Pi is 3.169834647 (error 0.90 percent).
Serial correlation coefficient is 0.365804 (totally uncorrelated = 0.0).
*/

// make sure standard library available
extern crate std;

use std::{
    io::Read,
    path::{Path, PathBuf},
};

#[cfg(doc)]
use crate as entest;

use entest::{Entest, EntestResult};

use clap::{Parser, CommandFactory};

#[derive(Debug, Parser)]
#[command(name="entest", version, author, about="entest (entropy test) is a program that applies tests to byte sequences stored in files or streams. A rust implementation similar to ent tool: https://www.fourmilab.ch/random/")]
pub struct Opt {
    /// Treat input as a stream of bits.
    #[arg(long, short='b')]
    bits: bool,

    /// Print occurrence counts.
    #[arg(long, short='c')]
    counts: bool,

    /// Fold upper to lower case letters.
    #[arg(long, short='f')]
    fold: bool,

    /// Terse output in CSV format.
    #[arg(long, short='t')]
    terse: bool,

    /// Print this message.
    #[arg(long, short='u')]
    usage: bool,

    /// optional file name.
    ///
    /// if not provided, then read from standard input.
    file: Option<PathBuf>,
}

fn from_reader<R: Read>(reader: &mut R) -> std::io::Result<EntestResult> {
    let mut buf = [0u8; 8192];
    let mut entest = Entest::new();
    loop {
        let len = reader.read(&mut buf)?;
        if len == 0 {
            // EOF?
            break;
        }

        entest.update(&buf[..len]);
    }

    Ok(entest.finalize())
}

fn from_stdin() -> std::io::Result<EntestResult> {
    let stdin = std::io::stdin();
    let mut stdin = std::io::BufReader::new(stdin.lock());
    from_reader(&mut stdin)
}

fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<EntestResult> {
    let file = std::fs::File::open(path)?;
    let mut file = std::io::BufReader::new(file);
    from_reader(&mut file)
}

fn result_main() -> std::io::Result<()> {
    let opt = Opt::parse();
    if opt.usage {
        Opt::command().print_help()?;
        return Ok(());
    }

    let er =
        if let Some(file) = opt.file {
            from_file(file)?
        } else {
            from_stdin()?
        };
    println!("{er}");

    Ok(())
}

fn main() {
    result_main().unwrap()
}
