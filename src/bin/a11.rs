// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
  id: i32,
  quantity: i32
}

fn display_id(gc: &GroceryItem) -> () {
  println!("The id of an item is: {:?}", gc.id);
}

fn display_quantity(gc: &GroceryItem) -> () {
  println!("There are {:?} left", gc.quantity);
}

fn main() {
  let cucumber = GroceryItem {
    id: 0,
    quantity: 11
  };

  display_id(&cucumber);
  display_quantity(&cucumber);
}
