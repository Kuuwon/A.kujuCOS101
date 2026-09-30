// Practice 7: The format! macro
fn main() {
    let k1 = "Yemisi".to_string();
    let k2 = " Shyllon".to_string();
    let k3 = " Museum".to_string();
    let k4 = " of".to_string();
    let k5 = " Art,".to_string();
    let k6 = " PAU".to_string();

    // format macro (six pieces in, so six {} placeholders)
    let k7 = format!("{} {} {} {} {} {}", k1, k2, k3, k4, k5, k6);
    // print output
    println!("\n {}", k7);
}
