fn main() {
    let b: (i32, bool, f64) = (120, false, 12.5);
    print(b);
}

fn print(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}
