enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // let x = value_in_cents(Coin::Quarter);

    // println!("{}", x);

    let y = Coin::Penny;
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
        Coin::Quarter => 25,
    }
}
