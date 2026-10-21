use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("could not remove file")
    println!("That bih ahh file was removed fn");
}
