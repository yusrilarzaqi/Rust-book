struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };

    println!("Username : {}", user1.username);
    println!("Email : {}", user1.email);

    user1.username = String::from("updatedusername");
    println!("Username : {}", user1.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
