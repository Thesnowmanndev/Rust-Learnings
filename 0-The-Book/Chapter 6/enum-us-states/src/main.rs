mod lib;

use std::io;
use strum::IntoEnumIterator;
use lib::*;

fn main() {
    let mut n : u8 = 1;
    println!("50 State Facts!");
    for state in UsStates::iter() {
        print!("#{}: ", n.to_string());
        state_fact(state);
        n += 1;
    }

    println!("Press the enter key to terminate.");
    io::stdin().read_line(&mut String::new()).unwrap();
}
