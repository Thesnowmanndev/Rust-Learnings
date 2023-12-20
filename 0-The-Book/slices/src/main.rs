use std::io;

fn main() {
    println!("Type two words."); // Asks user for words

    let mut input = String::new(); // Creates a variable to store the input

    io::stdin()
        .read_line(&mut input) // Reads the line and stores in variable above
        .expect("Failed to read line.");

    let word = first_word(&input); // Calls function to calculate how many characters are in the first word

    println!("There are {} characters in the first word of your input.", word); // Displays count
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item ==b' ' {
            return  i;
        }
    }

    s.len()
}