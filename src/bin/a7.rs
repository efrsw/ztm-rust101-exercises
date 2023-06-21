// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
   Red,
   Green,
   Blue,
   Yellow,
   White 
}

fn print_color(color: Colors) -> () {
  match color {
    Colors::Blue => println!("The color is Blue!"),
    Colors::Green => println!("The color is Green!"),
    Colors::Red => println!("The color is Red!"),
    Colors::White => println!("The color is White!"),
    Colors::Yellow => println!("The color is Yellow!"),
  }
}

fn main() {
  print_color(Colors::Blue);
}
