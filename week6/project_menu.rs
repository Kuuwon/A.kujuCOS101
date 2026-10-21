// Project: The Restaurant Menu
// Displays the menu, reads the food type and quantity, computes the total,
// and gives a 5% discount when the total is greater than N10,000.
use std::io;

fn main() {
    println!("===== PAU RESTAURANT MENU =====");
    println!("P - Poundo Yam / Edinkaiko Soup   N3,200");
    println!("F - Fried Rice & Chicken          N3,000");
    println!("A - Amala & Ewedu Soup            N2,500");
    println!("E - Eba & Egusi Soup              N2,000");
    println!("W - White Rice & Stew             N2,500");
    println!("===============================");

    // Read the food type
    println!("Enter food type (P, F, A, E or W):");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_uppercase();

    // Decide the price from the letter
    let (name, price): (&str, u32) = if food == "P" {
        ("Poundo Yam / Edinkaiko Soup", 3200)
    } else if food == "F" {
        ("Fried Rice & Chicken", 3000)
    } else if food == "A" {
        ("Amala & Ewedu Soup", 2500)
    } else if food == "E" {
        ("Eba & Egusi Soup", 2000)
    } else if food == "W" {
        ("White Rice & Stew", 2500)
    } else {
        println!("Invalid food type. Please run again and choose P, F, A, E or W.");
        return;
    };

    // Read the quantity
    println!("Enter quantity:");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    let quantity: u32 = match qty_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid quantity. Please enter a whole number.");
            return;
        }
    };

    // Compute the total
    let total = price * quantity;
    println!("\nOrder: {} x {}", quantity, name);
    println!("Total: N{}", total);

    // Discount: 5% when total is greater than N10,000
    if total > 10000 {
        let discount = total as f64 * 0.05;
        let final_total = total as f64 - discount;
        println!("Discount (5%): N{:.2}", discount);
        println!("Amount to pay: N{:.2}", final_total);
    } else {
        println!("No discount applied.");
        println!("Amount to pay: N{}", total);
    }
}
