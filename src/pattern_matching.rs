
struct Point {
    x: i32,
    y: i32,
}

pub fn pattern_matching() {
    let mut stack = Vec::new();  

    stack.push(1);
    stack.push(2); 
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value)  in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2,3);

    let point = &(3, 5);
    print_coordinates(point);

    // match literals

    let x = 1;
    match x {
        1 => println!("one"),
        2..=3 => {},
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) | None => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), //  notice how y is shadowed here
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }


}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}