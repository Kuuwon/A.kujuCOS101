fn main() {
    let b: (i32, bool, f64) = (28, true, 4.5);
    print(b);
}

fn print(x: (i32, bool, f64)) {
    println!("Inside print method");
    let (age, is_female, cgpa) = x;
    println!("Age is {}, isFemale? {}, cgpa is {}", age, is_female, cgpa);
}
