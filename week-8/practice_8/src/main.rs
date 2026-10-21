fn main() {
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993, "Kangchenjunga", 8586);

    println!("Original tuple = {:?}", mountain_heights);

    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;
    mountain_heights.4 = "Makalu";
    mountain_heights.5 = 8485;

    println!("Changed tuple = {:?}", mountain_heights);
}
