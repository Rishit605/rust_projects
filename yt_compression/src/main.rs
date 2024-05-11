extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

// Main Function
fn main() {
    if args().len() != 3 {
        eprintln!("Usage:`source` `target`"); // Error check fo the input.
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); // Takes the input file and performs the compression action.
    let output = File::create(args().nth(2).unwrap()).unwrap(); // Defines the output function and its action.

    let mut encoder = GzEncoder::new(output, Compression::default()); // This line initalizes the Encoder for the file compressor.

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len(),
    );

    println!("Target len {:?}", output.metadata().unwrap().len());
    println!("Elapsed {:?}", start.elapsed()); // Calculates the elapsed or up time.
}
