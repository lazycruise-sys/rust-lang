// Array - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    // unless there are going to be changes to the values of an immutable variable, don't assign a mut condition to it
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assign value
    numbers[2] = 20;

    // Get single value
    println!("My favourite number is {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}