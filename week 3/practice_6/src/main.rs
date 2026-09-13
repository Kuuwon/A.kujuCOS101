fn main() {
    // let interest: f32 = 8; would NOT compile:
    // error[E0308]: mismatched types. expected f32, found integer
    // The fix is to use a float literal instead:
    let interest: f32 = 8.0;

    println!("interest: {}", interest);
}
