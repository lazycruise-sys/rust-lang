// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct ColorTwo(u8, u8, u8);

// Struct with own functions
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get Full Name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn name_to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // Using the traditional struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0 
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Using the tuple struct
    let mut c2 = ColorTwo(255, 0, 0);
    c2.0 = 200;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    // Using struct with own functions
    let mut p = Person::new("Mary", "Doe");
    println!("Name: {}", p.full_name());
    p.set_last_name("Williams");
    println!("Name: {}", p.full_name());
    println!("Person Tuple: {:?}", p.name_to_tuple());
}