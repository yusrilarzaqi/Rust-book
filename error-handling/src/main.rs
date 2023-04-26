use std::fs::File;

fn main() {
    let greeting_file_result = File::open("src/hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:#?}", error),
    };

    println!("{:#?}", greeting_file);
}
