fn main() {
    // variables are declared with the 'let' keyword and are constant by default
    let name = "Kyle";

    // you can make a variable mutable by using the 'mut' keyword
    let mut greeting = "Hello, World!";

    // lets reassign the greeting variable
    greeting = "Hello";

    // the above variables 'name' and 'greeting' are string variables. You can define a variable using the : operator
    let proper_string : &str = "This is a string slice, the most primitive form of a string variable.";

    // Other variables
    let is_true : bool = true;

    let c : char = 'c';

    let float_32_bit : f32 = 1.337_f32;
    let float_64_bit : f64 = 1.33780085_f64;

    let smallest_integer : i8 = -128;
    let integer_16_bit :  i16 = -32768;
    let integer_32_bit : i32 = 2147483647; // 10 digits
    let integer_64_bit : i64 = 9223372036854775807; // 19 digits
    let integer_128_bit : i128 = -170141183460469231731687303715884105728; // 39 digits

    // cannot use signs with unsigned ints
    let unsigned_integer : u8 = 255; // MAX 255
    let unsigned_16_bit : u16 = 65535; // MAX 65,535
    let unsigned_32_bit : u32 = 4294967295; // MAX 4,294,967,295
    let unsigned_64_bit : u64 = 18446744073709551615; // 20 digits MAX 18,446,744,073,709,551,615
    let unsigned_128_bit : u128 = 340282366920938463463374607431768211455;
    // MAX 340,282,366,920,938,463,463,374,607,431,768,211,455
}
