use std::io;

fn main() {
    let mut user_input = String::new();
    let mut amount_of_inputs = 0;

    loop {
        if amount_of_inputs > 0 {
            user_input = "".parse().unwrap();
            println!(" ");
        }

        println!("Input a number:");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_input % 4 == 0 {
            println!("Input is divisible by 4.");
            break;
        } else if user_input % 3 == 0 {
            println!("Input is divisible by 3.");
            break;
        } else if user_input % 2 == 0 {
            println!("Input is divisible by 2.");
            break;
        } else {
            println!("Input is not divisible by 4, 3, or 2.");
            amount_of_inputs += 1;
        }
    }
}
