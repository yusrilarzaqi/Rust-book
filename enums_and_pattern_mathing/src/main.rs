#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
    // etc
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Albama);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("Count : {}", count);
}
