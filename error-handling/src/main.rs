use std::{io, fs};

fn read_username_from_flle() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let result = read_username_from_flle();

    println!("{}", result.unwrap());
}
