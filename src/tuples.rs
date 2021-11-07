// Tuples group together values of different types
// Holds a maximum of 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Femi", "Nigeria", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}