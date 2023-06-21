// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
  Orange,
  Apple,
  Chocolate
} 

struct Drink {
  flavor: Flavor,
  ounces: f32 
}

fn print_drink_info(drink: Drink) -> () {
  match drink.flavor {
    Flavor::Orange => println!("Orange, {:?}ml", drink.ounces),
    Flavor::Chocolate => println!("Chocolate, {:?}ml", drink.ounces),
    Flavor::Apple => println!("Apple, {:?}ml", drink.ounces),
  }
}

fn main() {
  let drink = Drink {
    flavor: Flavor::Apple,
    ounces: 50.0
  };

  print_drink_info(drink);

}
