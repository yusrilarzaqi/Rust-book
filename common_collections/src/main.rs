fn main() {
    let mut v = vec![100, 32, 57];

    for i in &v {
        *i += 50;
    }
}
