// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32),
}

fn main() {
    let tick = vec![
        Ticket::Vip(100, "Ivan".to_owned()),
        Ticket::Backstage(75, "Leha".to_owned()),
        Ticket::Standard(50),
    ];

    for t in &tick {
        match t {
            Ticket::Backstage(price, name) => {
                println!("Backstage. Price: {:?}, Name: {:?}", price, name)
            }
            Ticket::Vip(price, name) => {
                println!("VIP. Price: {:?}, Name: {:?}", price, name)
            }
            Ticket::Standard(price) => println!("Standard. Price: {:?}", price),
        }
    }
}
