enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny_value : u8 = value_in_cents(Coin::Penny);
    let nickel_value : u8 = value_in_cents(Coin::Nickel);
    let dime_value : u8 = value_in_cents(Coin::Dime);
    let quarter_value : u8 = value_in_cents(Coin::Quarter);

    println!("A Penny holds the value of {}.", penny_value);
    println!("A Nickle holds the value of {}.", nickel_value);
    println!("A Dime holds the value of {}.", dime_value);
    println!("A Quarter holds the value of {}.", quarter_value);
}
