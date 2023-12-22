use std::io;

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    let width_as_string = ask_width();
    let length_as_string = ask_length();
    let width_as_num: u32 = width_as_string.trim().parse().unwrap_or_else(|_| 0);
    let length_as_num: u32 = length_as_string.trim().parse().unwrap_or_else(|_| 0);

    let rect1 = Rectangle {
        width: width_as_num,
        length: length_as_num,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
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