// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn addition(n1: i32, n2: i32) -> i32 {
  return n1 + n2;
}

fn print_result(number: i32) -> () {
  println!("The result is {:?}", number);
}

fn main() {
  print_result(addition(5, 5));
}
