#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("i: {i:#?}");
    }

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <-- goes out of scope is freed here
}
