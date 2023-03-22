struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64
}

struct Color(u32, u32, u32); // tuple stuck

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Can have multiple impl blocks
impl Rectangle {
    // Associated functions

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return self.width >= rect.width && self.height >= rect.height;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

pub fn structs() {
    let mut user1 = User {
        _active: true,
        _username: String::from("pranay1010"),
        email: String::from("pranay@gmail.com"),
        _sign_in_count: 1
    };

    user1.email = String::from("pranay@outlook.com");

    let _user2 = User {
        email: String::from("another@email.com"),
        ..user1
    };

    let color = Color(1, 2, 43);
    let _first = color.0;

    // wont work -> println!("{}", user1.username);

    let _unit = AlwaysEqual;

    let rect = Rectangle{ width: 10, height: 5 };
    let area = rect.area();
    println!("Area of rectangle is {}", area);

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let _square = Rectangle::square(10);
}
