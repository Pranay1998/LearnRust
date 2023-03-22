// Each value in Rust has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value will be dropped

pub fn memory_ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // auto dealloc when goes out of scope

    let x = 5;
    let _y = x; // make a copy of value of x and store in y

    let s1: String = String::from("hello");
    let s2 = s1; // pointer to data on heap is copied

    // s1.push_str("test"); //-> error, since s1 is no longer valid
    let mut s3 = s2.clone();
    s3.push_str("This works because of clone");

    let s = String::from("hello"); //s comes into scope

    takes_ownership(s); // s's value moves into the function and is no longer valid here

    let _s1 = gives_ownership(); // s1 gets ownership

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);

    // Mutable Reference borrowing -> references are immutable by default

    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of {}, is {}", s1, len);

    // ** If you have a mutable references to a value, you can have no other references to that value

    let mut s = String::from("hello");

    let _r1 = &mut s;
    
    // Does not work //
    // let r2 = &mut s; 
    // println!("{}", r1);
    // println!("{}", r2);
    // ** If you have an immutable reference to a value, you can have no other mutable references to that value only immutable references
    
    // Reference's scope starts when a variable is created and end when it is last used
    // scope of mutable & mutable cannot overlap.
    // scope of mutable & immutable cannot overlap.
    let mut s = String::from("hello");

    // start of scopes of r1 & r2
    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);
    // variables r1 an r2 will not be used after this point

    // start of scope of r3
    let r3 = &mut s;
    println!("{}", r3);
    // end of scope of r3;

    // Dangling reference -> pointer that references a location in memory that was given to someone else
    // i,e -> freeing memory but keeping a pointer to that memory

    // This won't compile
    // let reference_to_nothing = dangle();

    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s -> trying to return reference to s but s goes out of scope
    // }

    slices();

}

fn takes_ownership(_some_string: String) {}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // return also transfers ownership
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string // takes a string and returns it back
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("string");
    s.len()
}

fn slices() {
    let s: String = String::from("hello world");

    let _hello: &str = &s[0..5]; // or [..5] // immutable borrow
    let world: &str = &s[6..11]; // or [6..] // immutable borrow

    // string slices must occur at valid utf-7 character boundaries, some characters take more than 1 byte -> this could lead to panic.

    // s.clear(); mutable borrow -> error

    println!("the first word is {}", world);

    // String literals

    let s = "Hello, world!";
    let s1: String = String::from("hello world");

    println!("First line in literal is {}", first_word(s));
    println!("First word in String is {}", first_word(&s1[..]));

    // same for array slices
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}