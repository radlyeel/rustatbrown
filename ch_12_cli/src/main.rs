// Target execution pattern: cargo run -- searchstring example-filename.txt
// A simple grep-like program to illustrate a few useful things, such
// as command-line arguments, file reading,

// For decomposition guidelines,
// https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects

// You Are Here:
// https://rust-book.cs.brown.edu/ch12-04-testing-the-librarys-functionality.html

use std::env;
use std::process;

// This show why crate naming is important!
use ch_12_cli::Config;
// With the 'data' directory alongside 'src', execute with
// cargo run -- searchstring ../data/poem.txt

fn main() {
    // gather cli args into a collection 'args'
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // For debugging
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = ch_12_cli::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
