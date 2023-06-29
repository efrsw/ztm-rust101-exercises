// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    age: i32, 
    name: String,
    color: String
}

impl Person {
    fn new(age: i32, name: String, color: String) -> Self {
        Self {
            age,
            name,
            color
        }
    }

    fn print_info(&self) {
        println!("{:?} favorite color is: {:?}", self.name, self.color);
    }
}

fn main() {
    let people = vec![
        Person::new(5, String::from("Bruh"), String::from("Blue")),
        Person::new(7, "Ivan".to_owned(), "Red".to_owned()),
        Person::new(12, "Max".to_owned(), "Green".to_owned())
    ];

    for p in people {
        if p.age <= 10 {
            p.print_info();
        }
    }
}
