use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn main() {
    greet_world();
    csv_data();
    different_storages();

    // rust_safety::dangling_reference();
    // rust_safety::race_condition();
    // rust_safety::buffer_overflow();
    // rust_safety::iterator_modification();
}

fn greet_world() {
    println!("Hello, world!");

    let another_greeting = "Goodbye, world...";
    let something_else = ":)";

    let regions = [another_greeting, something_else];

    for region in regions {
        println!("{}", &region);
    }
}

fn csv_data() {
    let penguin_data = "\
        common name,length (cm)
        Little penguin,33
        Funny penguin, 21
        Dingus,1
        Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().is_empty() {
            continue;
        }

        // Vec<_> infers what type goes into Vec.
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        // Only runs when running in debug mode.
        if cfg!(debug_assertions) {
            // Prints to stderr.
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        // if-let Ok allows for skipping any error cases when attempting to parse the number
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn different_storages() {
    let a = 10; // Values whose sizes are known at compile-time are stored on the stack.
    let b = Box::new(20); // Integer on the heap, also known as a boxed integer.
    let c = Rc::new(Box::new(30)); // Boxed integer wrapped within a reference counter.

    // Integer wrapped in an atomic reference counter and protected by a mutual exclusion lock.
    let d = Arc::new(Mutex::new(40));

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}

// Rust will prevent all of these things from happening.
#[allow(dead_code)]
mod rust_safety {

    pub fn dangling_reference() {
        #[derive(Debug)]
        enum Cereal {
            Barley,
            Millet,
            Rice,
            Rye,
            Spelt,
            Wheat,
        }

        let grains: Vec<Cereal> = vec![
            Cereal::Barley,
            Cereal::Millet,
            Cereal::Rice,
            Cereal::Rice,
            Cereal::Rye,
            Cereal::Spelt,
            Cereal::Wheat,
        ];

        // drop(grains); BUG: Will delete grains, making using grains later on impossible.
        println!("{:?}", grains);
    }

    pub fn race_condition() {
        let mut data = 100;

        // std::thread::spawn(|| {
        //     data = 500;
        // });
        //
        // std::thread::spawn(|| {
        //     data = 1000;
        // });
        // BUG: Rust does not allow multiple places in an application to have write access to data.

        println!("{}", data);
    }

    pub fn buffer_overflow() {
        let fruit = vec!["Red", "Blue", "Green"];

        // Will crash rather than assign an invalid memory location to a variable.
        let buffer_overflow = fruit[4];
        assert_eq!(buffer_overflow, "Yellow");
    }

    pub fn iterator_modification() {
        let mut letters = vec!["a", "b", "c"];

        for letter in letters {
            println!("{}", letter);
            // letters.push(letter.clone());
            // BUG: Rust doesn't allow the variable to be modified while in it's iteration block.
        }
    }
}
