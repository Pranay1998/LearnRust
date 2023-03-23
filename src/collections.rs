use std::collections::HashMap;

pub fn collections() {
    let _v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v2.push(6);

    let first: &mut i32 = &mut v2[0]; // panics if out of range, counts as immutable borrow

    *first = 32;

    for i in &mut v2 {
        println!("{i}");
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let _s3 = s1 + &s2; // s1 not valid anymore


    // use string.char() & string.bytes()

    let mut scores = HashMap::new();

    scores.insert("hello", 10);

    let val = scores.get("hello").copied().unwrap_or(0);





}