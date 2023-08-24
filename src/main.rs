use regex::Regex;
use std::env::{self, Args};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Configuration {
    re: Regex,
    file_path: String,
}

fn main() {
    let arguments = env::args();

    let config = manage_arguments(arguments).unwrap();
    let regex = config.re;
    let file_path = config.file_path;

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut result: Vec<String> = Vec::new();

    for line in reader.lines() {
        let haystack = &line.unwrap()[..];
        if regex.is_match(haystack) {
            result.push(haystack.to_string());
        }
    }

    println!("{:#?}", result);
}

fn manage_arguments(mut arguments: Args) -> Result<Configuration, &'static str> {
    arguments.next(); // First argument is executable path(?)
    let re = match arguments.next() {
        Some(expression) => Regex::new(&expression[..]).unwrap(),
        None => return Err("No regular expression provided."),
    };
    let file_path = match arguments.next() {
        Some(string) => string,
        None => return Err("No file path provided"),
    };

    Ok(Configuration {
        re: re,
        file_path: file_path,
    })
}
