use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //initialize variables

    let mut current_change: [f32; 4] = [0.00; 4];
    let mut index_of_current_change = 0;
    // set price to a random integer from 0 - 20 dollars
    let price = rand::thread_rng().gen_range(1.00..=20.00);

    // uncommnent this to print change to the console
    //println!("{price}");

    //assign a random value to each item in the current_change array
    while index_of_current_change < 4 {
        current_change[index_of_current_change] = rand::thread_rng().gen_range(1.00..=100.00);
        index_of_current_change += 1;

        // uncomment this to see the change
        //println!(
        //   "you have {:.2} quarters, {:.2} dimes, {:.2} nickels and {:.2}pennies",
        //    current_change[0], current_change[1], current_change[2], current_change[3]
        //);

        //convert each item to its worth in dollars
        let quarters = current_change[0] * 0.25;
        let dimes = current_change[1] * 0.10;
        let nickels = current_change[2] * 0.05;
        let pennies = current_change[3] * 0.01;

        let total_current_change = quarters + dimes + nickels + pennies;
        let item_affordable = if total_current_change >= price {
            true
        } else {
            false
        };

        //just print the output to the console to make it easier to read this
        if item_affordable == true {
            println!("true");
        } else {
            ("false");
        }
    }
}
