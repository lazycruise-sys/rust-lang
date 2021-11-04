/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
u - unsigned (not negative numbers)
i = assigned (both negative and positive numbers)
Floats: f32, f64
Boolen (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time.
// However, the compiler can usually infer what type we want to use based on the value and how to use it.
// The last comment is an example of the ability of Rust to afford you low-level control over performance with high-level convenience
// Default of, for example, let x = 1 would be "i32"
// Default of, for example, let y = 1.5 would be "f64"

pub fn run() {
    // Add explicit type
    let _y: i64 = 4521312121; // the underscore before the variable name lets Rust know that there is no intention of usage

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool =  true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Characters
    // Characters use the single quote literal and allow only unicodes
    let a1 = 'a';
    let face = '\u{1F600}'; // using the emoji unicode
    
    println!("{:?}", (_y, is_active, is_greater, a1, face));
}