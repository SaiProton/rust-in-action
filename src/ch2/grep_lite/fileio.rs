use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::Arg;
use clap::ArgAction;
use clap::Command;
use regex::Regex;

pub fn main() {
    // manual_reading();
    // iterator_reading();
    search_file();
}

fn manual_reading() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);

        line.truncate(0);
    }
}

fn iterator_reading() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}

fn search_file() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .action(ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("File to search")
                .action(ArgAction::Set)
                .required(true),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.get_one::<String>("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();

        if re.find(&line).is_some() {
            println!("{}", line);
        }
    }
}
