use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    // Uses println macro -- https://doc.rust-lang.org/std/macro.println.html
    println!("Welcome to the Guess the Number Game!");

    // Use rng trait from rand crate to generate random # -- https://docs.rs/rand/latest/rand/trait.Rng.html
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Create a integer and initialize as 0.
    let mut number_of_guesses: u32 = 0;

    // Loop until told to exit via break;
    loop {
        println!("Input a number from 1 to 100.");

        // Create a String to store the users input.
        let mut guess = String::new();

        // Use io trait & read_line function -- https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Use match control flow to check if number -- https://doc.rust-lang.org/std/keyword.match.html
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered: {guess}");

        // Uses cmp module for comparing values -- https://doc.rust-lang.org/std/cmp/index.html
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small."),
            Ordering::Greater => println!("Your guess is too large."),
            Ordering::Equal => {
                println!("You guessed the correct number!!!");
                println!("It took you {number_of_guesses} attempts to find the correct number.");
                break;
            }
        }

        number_of_guesses += 1;
    }
}
