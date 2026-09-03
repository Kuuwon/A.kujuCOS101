fn main() {
    let amounts = [
        2.0 * 450_000.00,
        1.0 * 1_500_000.00,
        3.0 * 750_000.00,
        3.0 * 2_850_000.00,
        1.0 * 250_000.00,
    ];

    let sum: f64 = amounts.iter().sum();
    let average = sum / amounts.len() as f64;

    println!("Total Sales: {:.2}", sum);
    println!("Average Sales: {:.2}", average);
}