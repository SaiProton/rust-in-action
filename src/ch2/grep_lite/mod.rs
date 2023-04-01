mod completed;
mod fileio;

use clap::{Arg, ArgAction, Command};
use regex::Regex;

pub fn main() {
    // search();
    // search_context();
    // search_regex();
    // search_arg();

    fileio::main();
    completed::main();
}

fn search() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{line_num}: {line}");
        }
    }
}

fn search_context() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    // Tags hold line numbers where matches occur.
    let mut tags: Vec<usize> = vec![];
    // ctx contains a vector per match to hold the context lines.
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    // Iterates through the lines, recording line numbers where matches are encountered.
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            // Vec::with_capacity reserves space for n items.
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    // When there are no matches, exits early.
    if tags.is_empty() {
        return;
    }

    // For each tag, at every line, checks to see if we are near a match.
    // If we are, adds that line to the relevant Vec<T> within ctx.
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // saturating_sub is subtraction that returns 0 on integer underflow.
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                // Copies line into a new String and stores that locally for each match.
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in &ctx {
        // Ref line informs the compiler that we want to borrow this value rather than move it.
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{line_num}: {line}");
        }
    }
}

fn search_regex() {
    // Unwrap unwraps a Result, crashing if an error occurs.
    let re = Regex::new("picture").unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);

        if contains_substring.is_some() {
            println!("{line}");
        }
    }
}

fn search_arg() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .action(ArgAction::Set)
                .required(true),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        if re.find(line).is_some() {
            println!("{line}");
        }
    }
}
