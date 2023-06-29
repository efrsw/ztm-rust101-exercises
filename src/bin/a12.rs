// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red, 
    Green
}

struct Dimensions {
    height: f64,
    width: f64,
}

struct Box {
    color: Color,
    weight: i64,
    dimensions: Dimensions
}

impl Box {
    fn new(color: Color, dimensions: Dimensions, weight: i64) -> Self {
        return Self {
            color: color, 
            weight: weight,
            dimensions: dimensions
        }
    }

    fn print_box(&self) -> () {
        println!("Weight: {:?}", self.weight);
        println!("Dimensions: {:?}, {:?}", self.dimensions.height, self.dimensions.width);
        
        match self.color {
            Color::Red => println!("Color is red"),
            Color::Green => println!("Color is green"),
        };
    }
}

impl Dimensions {
    fn new(height: f64, width: f64) -> Self {
        Self {
            height,
            width
        }
    }
}

fn main() {
    let b = Box::new(Color::Green, Dimensions::new(50.0, 50.0), 75);
    b.print_box();
}
