use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let vikings = HashMap::from([
        (Viking::new("Ibex", "Iceland"), 34),
        (Viking::new("Wombat", "Findland"), 25),
        (Viking::new("Olaf", "Norway"), 40)
    ]);

    for (viking, health) in &vikings {
        println!("{viking:?} has {health} hp");
    }
}
