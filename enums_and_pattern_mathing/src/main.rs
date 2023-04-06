#[derive(Debug)] // so we can inspect the state in a minute
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
    // let x = value_in_cents(Coin::Quarter);

    // println!("{}", x);

    let y = Coin::Quarter(UsState::Alaska);
    let x = value_in_cents(y);

    println!("{}", x);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        }
    }
}
