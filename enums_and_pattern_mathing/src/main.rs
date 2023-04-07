fn main() {
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(),
	}
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num: u8) {}
