use std::{env, process};

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Using unwrap_or_else allows us to define some custom, non-panic! error handling.
    // If Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping.
    // If value is an Err value, this method calls the code in the closure, which is an anonymous function we define and
    // pass as an argument to unwrap_or_else.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1)
    }
}




