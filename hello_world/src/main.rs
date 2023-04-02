#[derive(Default)]
struct Person {
    first_name: String,
    last_name: String,
}   

impl Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }

    fn first_name_mut(&mut self) -> &mut String {
        &mut self.first_name
    }
    fn last_name_mut(&mut self) -> &mut String {
        &mut self.last_name
    }
}

fn main() {
    let mut yusril = Person::default();

    *yusril.first_name_mut() = String::from("Yusril");
    *yusril.last_name_mut() = "Arzaqi".into();

    println!("first name : {}", yusril.first_name());
    println!("last name : {}", yusril.last_name());

}
