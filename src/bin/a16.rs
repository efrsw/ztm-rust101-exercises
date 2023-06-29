// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Assignment {
    name: String,
    locker: Option<i32>
}

impl Assignment {
    fn print_details(&self) {
        match self.locker {
           Some(mark) => println!("Name: {:?}, Assignment: {:?}", self.name, mark),
           None =>  println!("Name: {:?}, Assignment: None", self.name)
        }
    }
}

fn main() {
    let a = Assignment {
        name: "Assignment name".to_owned(), 
        locker: None
    };

    a.print_details();
}
