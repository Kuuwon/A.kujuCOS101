fn main() {
    let t: (&str, f32, u8) = ("Rust", 2.71, 200);
    println!("Tuple contents = {:?}", t);

    let t = ("Rust", "cool", 250);
    println!("Tuple contents = {:?}", t);

    println!("Value at index 0 = {}", t.0);

    println!("Value at index 1 = {}", t.1);

    println!("Value at index 2 = {}", t.2);
}
