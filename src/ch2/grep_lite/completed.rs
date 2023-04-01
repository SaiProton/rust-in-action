use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: &Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        if re.find(&line).is_some() {
            println!("{}", line);
        }
    }
}

pub fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .args([
            Arg::new("pattern")
                .help("The pattern to search for")
                .action(ArgAction::Set)
                .required(true),
            Arg::new("input")
                .help("File to search")
                .action(ArgAction::Set)
                .required(false),
        ])
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let binding = String::from("-");
    let input = args.get_one::<String>("input").unwrap_or(&binding);

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, &re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, &re);
    }
}
