fn main() {
    let s1 = String::from("My length is 16.");

    let len = calculate_length(&s1);    // The &s1 syntax lets us create a reference that refers to the value of
                                                // s1 but does not own it.

    println!("The length of '{}' is {}.", s1, len);

    println!("{}", s1); // Showing string wasn't dropped because calculate_length doesn't have ownership.
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.