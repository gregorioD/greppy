use regex::Regex;
use std::env::Args;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Configuration {
    re: Regex,
    file_path: String,
}

#[derive(Debug)]
enum ConfigError {
    Arguments,
    Regex,
    FilePath,
}

#[derive(Debug)]
struct MyError {
    error: ConfigError,
}

impl MyError {
    fn new(error_type: ConfigError) -> MyError {
        MyError { error: error_type }
    }
}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error {
            ConfigError::Arguments => {
                write!(f, "There has been an innapropiate amount of argumets.")
            }
            ConfigError::FilePath => write!(f, "There has been a problem with the file path."),
            ConfigError::Regex => {
                write!(f, "There has been a problem with the regular expression.")
            }
        }
    }
}

impl Configuration {
    pub fn build(mut arguments: Args) -> Result<Configuration, Box<dyn Error>> {
        if arguments.len() < 3 {
            return Err(Box::new(MyError::new(ConfigError::Arguments)));
        }
        arguments.next(); // First argument is executable path(?)

        let re = match arguments.next() {
            Some(expression) => Regex::new(&expression[..])?,
            None => return Err(Box::new(MyError::new(ConfigError::Regex))),
        };
        let file_path = match arguments.next() {
            Some(string) => string,
            None => return Err(Box::new(MyError::new(ConfigError::FilePath))),
        };

        Ok(Configuration {
            re: re,
            file_path: file_path,
        })
    }
}

pub fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
    let regex: Regex = config.re;
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    let mut result: Vec<String> = Vec::new();

    for line in reader.lines() {
        let haystack = &line.unwrap()[..];
        if regex.is_match(haystack) {
            result.push(haystack.to_string());
            println!("{}", haystack);
        }
    }
    println!("Number of times regex has been found: {}", result.len());

    Ok(())
}
