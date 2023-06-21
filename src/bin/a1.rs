// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name(name: &str) -> () {
   println!("My first name is {:?}", name); 
}


fn main() {
    first_name("Egor");
    first_name("Fursov");
}
