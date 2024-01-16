use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // function returns a Result with a Config instance in the success case and a &'static str in the error case
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // when the user doesn’t pass enough arguments, we return an Err value
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // wrapped the Config return value in an Ok
        Ok(Config { query, file_path })
    }
}