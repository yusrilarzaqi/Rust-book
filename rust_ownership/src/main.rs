#![allow(dead_code)]

fn main() {
    // let mut s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}", s1, len);

    // change(&mut s1); // create mutable reference

    let s = String::from("hello");

    let r1 = &s; // cannot borrowed mut twice
    let r2 = &s;

    println!("r1: {}, r2: {}", r1, r2);
}

fn change(s: &mut String) {
    s.push_str(", world"); // cannot mutate bororwed variable
}

fn some_func(s: &String) -> (&String, String) {
    (s, s.to_owned())
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn gives_ownership() -> String {
    String::from("hi mom")
}

fn take_and_gives_back(some_string: String) -> String {
    some_string
}

fn mbuh() {
    let s = String::from("hello"); // s is valid here
    take_ownership(&s); // s value move into the function
                        // s is no longer valid
    println!("{s}");

    let x = 5; // x is valud here
    makes_copy(x); //  x value is copy
                   // use x after word
    println!("{x}");
} // x goes out of scope, then s
fn take_ownership(some_string: &String) {
    println!("{some_string}");
} // somestring goes out of scope, `drop` is called

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // somegoes out of scope, `drop` is called

fn variable_scope() {
    {
        // `s` is not yet declared
        let s = "string literal"; // `s` is valid from this point forward
                                  // do with stuff `s`
        println!("{}", s);
    } // `s` is no longger valid
}

fn memory_savety() {
    // let s = "String";
    let mut s = String::from("String"); // s is string literal
    s.push_str(", Literal");

    println!("{s}");
}

fn borrowing() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    // let s2 = s1; // moved
    let s2 = s1.clone();
    println!("s1: {}\ns2: {}", s1, s2);
}
