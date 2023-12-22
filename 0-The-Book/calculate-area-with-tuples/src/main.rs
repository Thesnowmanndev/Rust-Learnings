use std::io;

fn main() {
    let width_as_string = ask_width();
    let length_as_string = ask_length();
    let width_as_num: u32 = width_as_string.trim().parse().unwrap_or_else(|_| 0);
    let length_as_num: u32 = length_as_string.trim().parse().unwrap_or_else(|_| 0);

    let rect1 = (width_as_num, length_as_num);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
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

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}