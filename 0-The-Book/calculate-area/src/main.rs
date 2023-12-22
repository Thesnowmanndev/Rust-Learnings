use std::io;

fn main() {
    let width_as_string = ask_width();
    let length_as_string = ask_length();

    let width_as_num: u32 = width_as_string.trim().parse().unwrap_or_else(|_| 0);
    let length_as_num: u32 = length_as_string.trim().parse().unwrap_or_else(|_| 0);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width_as_num, length_as_num)
    );
}

fn ask_length() -> String {
    let mut _string = String::new();
    println!("Type in the length:");
    io::stdin().read_line(&mut _string).expect("Failed to read line.");

    _string
}

fn ask_width() -> String {
    let mut _string = String::new();
    println!("Type in the width:");
    io::stdin().read_line(&mut _string).expect("Failed to read line.");

    _string
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}