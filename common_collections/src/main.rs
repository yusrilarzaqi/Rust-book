fn main() {
    accessing_values_in_a_hash_map();
}

#[allow(dead_code)]
fn push_method() {
    let mut s = String::from("lo");
    s.push('l');
    println!("s: {s}");
}

#[allow(dead_code)]
fn push_str_method() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {s2}");
}

#[allow(dead_code)]
fn concatenation_with_operator() {
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and ccan no longer be used
    //
    // println!("s3: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {s}");
}

#[allow(dead_code)]
fn concatenation_with_format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}");
}

#[allow(dead_code)]
fn indexing_into_strings() {
    // let s1 = String::from("Hello");
    // let h = s1[0];

    // let hello = String::from("Здравствуйте");
    // if let Some(chr) = hello.chars().nth(1) {
    //     println!("{chr}");
    // } else {
    //     println!("idk");
    // }
}

#[allow(dead_code)]
fn slicing_string() {
    let hello = "Здравствуйте";
    let s = &hello[..3];
    println!("s: {s}");
}

#[allow(dead_code)]
fn methods_for_iterating() {
    // for c in "Зд".chars() {
    //     println!("{c}");
    // }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

#[allow(dead_code)]
fn creating_a_new_hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:#?}");
}

fn accessing_values_in_a_hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

}
