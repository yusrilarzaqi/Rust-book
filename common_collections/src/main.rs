fn main() {}

fn push_method() {
    let mut s = String::from("lo");
    s.push('l');
    println!("s: {s}");
}

fn push_str_method() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {s2}");
}
