use std::fs::File;
use std::io::Write;

fn main() {
    let lager = ["33 Export","Desparados,'Goldberg","Gulder","Heineken","Star"];
    let stout = ["Legend","Turbo King","Williams"];
    let non_alcoholic = ["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    let mut file =std::fs::File::create("cb.txt").expect("could not create file");
    file.write_all("lager\n".as_bytes()).expect("write failed");
   file.write_all("stout\n".as_bytes()).expect("write failed");
   file.write_all("non-alcoholic\n".as_bytes()).expect("write failed");
   println!("\ncb.txt was created");

}

