use greppy::{run, Configuration};
use std::{env, process};

fn main() {
    let arguments = env::args();

    let configuration = Configuration::build(arguments).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    match run(configuration) {
        Err(e) => eprintln!("Error: {}", e),
        Ok(_) => (),
    }
}
