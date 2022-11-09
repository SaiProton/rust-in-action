mod grep_lite;
mod mandelbrot;

use num::complex::Complex;
use std::time::{Duration, Instant};

pub fn execute() {
    // different_bases();
    // comparison_between_types();
    // add_floats();
    // complex_numbers();
    // increment_benchmark();
    // print_is_even(10);
    // match_values();
    //
    // mandelbrot::execute();
    grep_lite::execute();
}

fn different_bases() {
    let three = 0b11; // binary
    let thirty = 0o36; // octal
    let three_hundred = 0x12C; // dexadecimal

    println!("decimal: {} {} {}", three, thirty, three_hundred);
    println!("binary: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("octal: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("hexadecimal: {:x} {:x} {:x}", three, thirty, three_hundred);
}

fn comparison_between_types() {
    let a: i32 = 10;
    let b: u16 = 100;

    // Using try_into instead of `as` to handle cases where the conversion may fail.
    let b_ = b.try_into().unwrap();

    // It is safest to cast the smaller type to a larger one (promotion).
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

fn add_floats() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert_eq!(abc.0 + abc.1, abc.2);
    // assert_eq!(xyz.0 + xyz.1, xyz.2);
    // BUG: Will crash! Equality comparisons between floating point numbers should be avoided.

    // Instead, we check if it's within an acceptable margin of the result. (epsilon)
    assert!(xyz.0 + xyz.1 - xyz.2 <= f64::EPSILON);
}

fn complex_numbers() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

fn increment_benchmark() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
}

fn print_is_even(n: i32) {
    // Rust is an expression-based language.
    let description = if n % 2 == 0 { "even" } else { "odd" };

    // This also works.
    // let description = match n % 2 == 0 {
    //     true => "even",
    //     false => "odd",
    // };

    println!("{} is {}", n, description);
}

fn match_values() {
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}
