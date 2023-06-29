// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert(5, "Chair".to_owned());
    stock.insert(3, "Bed".to_owned());
    stock.insert(2, "Table".to_owned());
    stock.insert(0, "Couche".to_owned());

    let mut summary = 0;

    for (key, value) in stock.iter() {
        match key {
            0 => println!("An item {value:} is out of stock"),
            _ => println!("Entry: {value:}, Amount: {key:}")
        }

        summary += key;
    }

    println!("Total amount of goods in stock: {summary:}");
}
