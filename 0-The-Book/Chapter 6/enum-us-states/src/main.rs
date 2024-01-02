use std::io;

// This crate is not from the book
use strum::IntoEnumIterator;

mod lib;
use lib::*;

fn main() {
    let mut n : u8 = 1;
    println!("50 State Facts!");

    // This iter() function is not from the book. 
    // See strum crate -- https://crates.io/crates/strum
    for state in UsStates::iter() {
        print!("#{}: ", n.to_string());
        state_fact(state);
        n += 1;
    }

    // Waits for user input to exit terminal.
    println!("Press the enter key to terminate.");
    io::stdin().read_line(&mut String::new()).unwrap();
}
