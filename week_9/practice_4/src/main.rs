use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Babby keem\n"
        .as_bytes()).expect("write failed");
    
    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "cannot open file");
    file.write_all("\n Hello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document."
        .as_bytes()).expect("write failed");
    println!("file append success");
}
