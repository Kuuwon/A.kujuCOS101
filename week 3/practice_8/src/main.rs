fn main() {
    let fees = 25_000; // fees is bound once, immutable by default

    // fees = 30_000; would NOT compile:
    // error[E0384]: cannot assign twice to immutable variable `fees`
    // help: consider making this binding mutable: `mut fees`

    println!("fees: {}", fees);
}
