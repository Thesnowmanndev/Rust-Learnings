// See -- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s = String::from("I once lived in main, now I live in takes_ownership.");

    takes_ownership(s); // transfers the value of s to some_string and drops s

    let x = 69;

    makes_copy(x);

    // println!("{}", s); -- does not work because the value was moved to the takes_ownership function then dropped
    // after code execution. That is why takes_ownership is named that way. Line 10 = error[E0382]
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}, nice!", some_integer);
}