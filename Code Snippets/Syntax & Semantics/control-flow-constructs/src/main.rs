fn main() {
    // While loop -- https://doc.rust-lang.org/std/keyword.while.html
    let mut counter : u8 = 0;
    while counter < 10 {
        println!("{counter}");
        counter += 1;
    }

    // For loop -- https://doc.rust-lang.org/std/keyword.for.html
    println!(" ");
    for i in 0..5 {
        println!("{}", i * 2)
    }

    // Match -- https://doc.rust-lang.org/std/keyword.match.html
    println!(" ");
    let a_number = Some(10);
    match a_number {
        Some(x) if x <= 5 => println!("0 to 5 num = {x}"),
        Some(x @ 6..=10) => println!("6 to 10 num = {x}"),
        None => panic!(),
        _ => panic!(),
    }

    // If / Else -- https://doc.rust-lang.org/std/keyword.if.html
    println!(" ");
    let greeting = "Hello, World.";
    if greeting == "Hello, World." {
        println!("Default Program.")
    } else {
        println!("Changes have been made.")
    }
}
