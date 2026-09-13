fn main() {
    let int_with_separator = 50_000; // underscores instead of commas
    let float_with_separator = 11_000.555_001; // works inside floats too

    println!("int_with_separator: {}", int_with_separator);
    println!("float_with_separator: {}", float_with_separator);
}
