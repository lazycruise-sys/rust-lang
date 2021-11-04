// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // variable assignment
    let name =  "Femi";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);

    // unless you had the keyword "mut", you can't assign
    // a value twice to immutable variable (as it is the default)
    age = 38;

    println!("My name is {} and I am {}", name, age);

    // unused assignments are warnable and if in such case
    // use the _ as a prefix before the variable_name.
    // also a change in value assignment to a variable 
    // without the former value being used is warnable
    // it is good practise to use the variables

    // Define constant
    const ID: i32 = 301;

    println!("ID: {}", ID);

    // you have to had a type to a 'constant' variable
    // if constant not used, the below error shows;
    // constant is never used: `ID`
    // note: `#[warn(dead_code)]` on by default

    // Assign multiple variables
    let ( my_name, my_age ) = ("Brad", 38);
    println!("{} is {}", my_name, my_age);
}