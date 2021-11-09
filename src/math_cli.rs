use std::env;

pub fn run() {
    // holds the argument within a vector variable
    let args: Vec<String> = env::args().collect();
    let len_args = args.len();
    let y = &args[2..len_args];
    let command = args[1].clone();

    // conditional statement that checks for the command type
    let mut count = 1;
    for x in y {
        let new_x = x.parse::<i32>().unwrap();
        if command == "multiply" {
            count *= new_x;
        } else if command == "add" {
            count += new_x;
        } else if command == "substract" {
            count -= new_x;
        } else {
            println!("Something is wrong! Check command name - The only allowed names are 'add' & 'multiply'.");
            println!("Working to add 'divide'.");
        }
    }

    println!("{}", count);
}