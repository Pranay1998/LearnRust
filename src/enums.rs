enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    _kind: IpAddrKind,
    _address: String
}

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::_Move { x, y } => println!("Move to {} {}", x, y),
            Message::_Write(_) => println!("Write"),
            Message::_ChangeColor(_, _, _) => println!("Change color"),
        }
    }
}

enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter
}

pub fn enums() {
    let _home = IpAddr {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1")
    };

    let _loopback = IpAddr {
        _kind: IpAddrKind::V6,
        _address: String::from("::1")
    };

    let _home = IpAddress::V4(127, 0, 0, 1);
    let _loopback = IpAddress::V6(String::from("::1"));

    let message = Message::Quit;
    message.call();

    let _x: i8 = 5;
    let _y: Option<i8> = Some(8);
    let _z: Option<i8> = None;

    let _coin = Coin::Quarter;
}

fn _value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter => 25,
    }
}