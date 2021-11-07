// Primitive str: Immutable fixed-length string somewhere in memory
// String: Growable, heap-allocated data structure and is used when there is need to modify or own string data

pub fn run() {

    // primitive string
    let _hello = "Hello"; // type:=> &str

    // string
    let mut tenses = String::from("Hello, ");

    // Get Length
    println!("Length: {}", tenses.len());

    // Appends a give character to the end of the string
    tenses.push('A');

    // Appends a given string to the end of the string
    tenses.push_str("ndrew.");

    // Capacity in bytes
    println!("{}", tenses.capacity());

    // Check if empty
    println!("Is Empty: {}", tenses.is_empty());

    // Check if the given pattern matches a sub-slice of a string slice.
    // patterns and string slice match must be of the same case
    println!("Contain 'And': {}", tenses.contains("And"));

    // Replace
    println!("Replace: {}", tenses.replace("Andrew", "There"));

    // Loop through string by whitespace
    for word in tenses.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('s');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    
    println!("{}", s);

}