fn main() {
    let mut x = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    let y = exercises1(&mut x);
    println!("y: {y}");
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

#[allow(dead_code)]
fn accessing_values_in_a_hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

#[allow(dead_code)]
fn hash_map_and_ownership() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get
    // println!("{field_name}: {field_value}");
}

#[allow(dead_code)]
fn overwriting_a_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".to_string(), 25);

    println!("scores: {scores:#?}");
}

#[allow(dead_code)]
fn adding_a_key_and_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:#?}");
}

#[allow(dead_code)]
fn updating_a_value() {
    use std::collections::HashMap;

    let text = "hello world wonderful world hello";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}

#[allow(dead_code)]
fn exercises1(list: &mut Vec<i32>) -> f32 {
    list.sort();
    list.iter().sum::<i32>() as f32 / list.len() as f32
}

#[allow(dead_code)]
