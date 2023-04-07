fn main() {
    let dice_roll = 3;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => (),
	}
}

fn add_fancy_hat() {
    println!("Fancy hat added")
}
fn remove_fancy_hat() {
    println!("Fancy hat removed")
}
