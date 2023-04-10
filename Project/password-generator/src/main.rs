use rand::Rng;

fn main() {
    let password_length = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "12".to_string())
        .parse::<usize>()
        .unwrap_or(12);

    let password = generate_password(password_length);

    println!("{password}");
}

fn generate_password(password_length: usize) -> String {
    (0..password_length)
        .map(|_| rand::thread_rng().gen_range(b'A'..b'Z') as char)
        .collect()
}
