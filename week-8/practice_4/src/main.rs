fn main() {
    let name = vec!["Mary","Sam","Sally","Gred","Ade","Mark","June","Ife","Tola"];

    let age = vec![16,17,19,22,20,21,18,23,24];

    print!("\nAge allocation\n");

    for i in 0..age.len() {
        println!("{} is {} years old", name[i], age[i]);
    }
}
