fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square piexels.",
        area(rect1)
    )
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
