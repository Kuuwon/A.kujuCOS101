fn value(n: Option<&char>) {
    println!("Element of vector {:?}", n);
}

fn main() {
    let v = vec!['R','U','S','T','A','C','E','A','N','S'];

    let mut input1 = String::new();
    println!("\nEnter an index value between (0-9)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    let index: usize = input1.trim().parse().expect("Invalid input");

    let ch = v.get(index);
    value(ch);
}
