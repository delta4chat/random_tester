//! the `entest` command line tools.

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
       Entropy = 7.980627 bits per character.

       Optimum compression would reduce the size
       of this 51768 character file by 0 percent.

       Chi square distribution for 51768 samples is 1542.26, and randomly
       would exceed this value 0.01 percent of the times.

       Arithmetic mean value of data bytes is 125.93 (127.5 = random).
       Monte Carlo value for Pi is 3.169834647 (error 0.90 percent).
       Serial correlation coefficient is 0.004249 (totally uncorrelated = 0.0).
*/

// make sure standard library available
extern crate std;

use std::{
    io::Read,
    path::Path,
};

use entest::{Entest, EntestResult};

use clap::Parser;

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

fn main() {
    //println!("hello world {:?}", Entest::new());
    let r = from_stdin().unwrap();
    println!("result: {}", r);
}
