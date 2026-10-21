fn main() {
    let v = vec!['C','O','M','P','U','T','E','R','S'];

    let mut input1 = String::new();

    println!("Enter an index value between (0-8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index: usize = input1.trim().parse().expect("Invalid input");

    let ch: char = v[index];

    print!("{} is the character for index [{}]\n", ch, index);
}
