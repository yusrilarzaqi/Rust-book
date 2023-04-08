use crate::garden::vegatables::Aspargus;

pub mod garden;

fn main() {
    let plant = Aspargus {};
    println!("I'm growing {:?}!", plant);
}
