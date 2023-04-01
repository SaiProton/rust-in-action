#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

// A function for parsing a line and converting it to semi-structurd data.
fn parse_log(line: &str) -> (Event, Message) {
    // Splits line into two parts from the first space.
    let parts: Vec<_> = line.splitn(2, ' ').collect();

    // If there aren't two parts, returns an error.
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    // Assigns each part of parts to a variable to ease future use.
    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

pub fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{parse_result:?}");
    }
}
