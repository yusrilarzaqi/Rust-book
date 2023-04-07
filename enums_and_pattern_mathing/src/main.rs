fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let x = Some(2);
    let y = plus_one(x);

    println!("{:#?}", y);
}
