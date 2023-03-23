mod guessing_game;
mod ownership;
mod structs;
mod enums;
mod collections;

const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

fn main() {
    if false {
        print_to_console();
        variables_and_mutability();
        shadowing();
        compound_types();
        println!("Return value of function is {}", statements_and_expressions());
        control_flow();
    }
    if false { guessing_game::guessing_game(); }
    if false { ownership::memory_ownership(); }
    if false { structs::structs(); } 
    if false { enums::enums(); }
    if true { collections::collections(); }
}

fn print_to_console() {
    println!("Hello, world");
}

fn variables_and_mutability() {
    let x: i32 = 5; // Immutable
    println!("The value of x is: {x}");
    let mut x: i32 = 6; // Mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Three hours is {THREE_HOURS_IN_SECONDS} seconds.");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The string contains {spaces} spaces");
}

fn compound_types() {
    let tup: (i32, bool) = (500, false);
    let (x, y) = tup; // Desctructuring
    println!("The value of x is {x} and y is {y}");

    let test_array: [i32; 5] = [1, 2, 4, 5, 6]; // alternative - [value; size]
    print!("Array - ");
    for val in test_array {
        print!("{val} ");
    }
    println!();
}

fn statements_and_expressions() -> i32 {
    let y = 6; // statement
    let x = { // expression
        let z = 3;
        z + 1 // expressions do not include ending semicolon
    };
    println!("Value {y} assigned in statement, value {x} assigned in expression");
    1 // or return 1;
}

fn control_flow() {
    let x = 3*21;

    let result = if x < 5 {
        "x is less than 5"
    } else {
        "x is equal to or greater than 5"
    };
    println!("{}", result);

    let mut counter = 0;

    let result = 'counting_up: loop {
        if counter == 10 {
            break 'counting_up counter;
        }
        counter = counter + 1;
    };
    println!("The reslut of the loop is {result}");

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("Liftoff!");
}