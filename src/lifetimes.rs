struct ImportantExcerpt<'a> {
    _part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }
}

pub fn lifetimes() {
    // Lifetimes aim to prevent dangling references -> pointers to deallocated memory
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentance = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        _part: first_sentance
    };

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}