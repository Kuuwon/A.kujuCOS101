use std::io ::Write;

fn main() {
  
  let announce = "Week  9 - Rust File Input & Output\n";
  let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n"
        .as_bytes()).expect("write failed");
    file.write(announce.as_bytes()).expect("write failed");
    file.write(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");










}
