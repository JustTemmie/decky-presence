use std::fs::File;

fn main() {
    File::create("./text.txt").unwrap();

    println!("Hello, world!");
}
