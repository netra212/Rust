// Basic Pattern Matching.
// The match expression compares a value against patterns.

// structs
struct Point {
    x: i32,
    y: i32,
}

// enums
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Color: rgb({}, {}, {})", r, g, b);
        }
    }
}

// Destructure in parameters.
fn print_coordinates(&(x, y): (i32, i32)) {
    println!("({}, {})", x, y);
}

// with structs. 
struct Point {x: i32, y: i32}

fn distance_from_origin(
    Point { x, y}: &Point
) -> f64 {
    ((x * x + y * y) as f64).sqrt()
}

fn main() {
    let number = 13;

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("Prime"),
        13..=19 => println!("Teen"),
        _ => println!("Other"),
    }

    // Match is an expression - it returns a value.
    let description = match number {
        1 => "One",
        2 => "Two",
        _ => "many",
    };

    println!("Description: {}", description);

    //-----------------
    // Destructuring.
    // Patterns can destructure complex types.
    // Tuples.
    //-----------------
    let pair = (0, -2);

    match pair {
        (0, y) => println!("On y-axis at {}", y),
        (x, O) => println!("On x-axis at {}", x),
        (x, y) => println!("At ({}, {})", x, y),
    }

    // Nested tuples.
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;

    println!("{} {} {} {}", a, b, c, d);

    //-----------------
    // Structs
    //-----------------
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }

    // Shorthand when variable name matches field name.
    let Point { x, y } = point;
    println!("Destructured: x={}, y={}", x, y);

    // Ignores field with..
    let Point { x, .. } = point;
    println!("Only x:{}", x);

    //-----------------
    // Enums.
    //-----------------
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write(String::from("hello")));
    process(Message::ChangeColor(255, 128, 0));

    //-----------------
    // Pattern Guards.
    //-----------------
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than 5: {}", x);
        Some(x) if x >= 5 => println!("5 or more: {}", x),
        None => println!("None"),
        _ => unreachable!(),
    }

    // Multiple conditions. 
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Opposites"),
        (x, _y) if x % 2 == 0 => println!("First is even"),
        _ => println!("Other"),
    }

    //------------------------
    // If let and while let. 
    //------------------------
    let config_max = Some(3u8);

    // Instead of match for single case. 
    if let Some(max) = config_max {
        println!("Maximum is {}", max);
    }
    // With else. 
    let value: Option<i32> = None;
    if let Some(v) = value {
        println!("Got: {}", v);
    } else {
        println!("Nothing");
    }

    // while let for iterations. 
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    // let else for early return. 
    fn process(opt: Option<i32>) -> i32 {
        let Some(value) = opt else {
            return 0;
        };
        value * 2;
    }

    //------------------------
    // Binding with @
    //------------------------
    let num = Some(4);
    match num {
        Some(n @ 1..=5) => println!("Got {} (1-5)", n),
        Some(n @ 6..=10) => println!("Got {} (6-10)", n),
        Some(n) => println!("Got {} (other)", n), 
        None => println!("None"),
    }

    // Binding with struct patterns.
    struct Point {
        x: i32, 
        y: i32
    }

    let p = Point {
        x: 0,
        y: 5
    };

    match p {
        Point {x : 0, y: y_val @ 1..=10 } => {
            println!("On y-axis at {} 91-10)", y_val);
        },
        Point {x, y} => println!("At ({}, {})", x, y),
    }

    //------------------------
    // Multiple Patterns
    //------------------------
    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        3 | 4 | 5 => println!("Three to five"),
        _ => println!("Other")
    }

    // With enums. 
    enum Color {
        Red, Green, Blue, Yellow, Cyan, Magenta
    }

    let color = Color:Blue;
    match color {
        Color::Red | Color::Green | Color::Blue => {
            println!("Primary color");
        }
        Color::Yellow | Color::Cyan | Color::Magenta => {
            println!("Secondary color");
        }
    }

    //------------------------
    // Range Patterns
    //------------------------
    let x = 5;
    match x {
        1..=5 => println!("One through five"),
        6..=10 => println!("Six through ten"),
        _ => println!("Other"),
    }

    // Characters ranges. 
    let c = 'c';
    match c {
        'a'..='j' => println!("Early Letter"), 
        'k'..='z' => println!("Late Letter"), 
        _ => println!("Other"),
    }

    //------------------------
    // Ignoring Values
    //------------------------
    // Ignore single value. 
    let pair1 = (1, 2);
    let (first, _) = pair1;
    println!("First: {}", first);

    // Ignore multiple values. 
    let numbers = (1, 2, 3, 4, 5);
    let (first, .., last) = numbers;
    println!("First: {}, Last: {}", first, last);

    // Ignore remaining struct fields. 
    struct Person {
        name: String, 
        age: u32, 
        address: String,
    }

    let person = Person {
        name: String::from("Alice"), 
        age: 30, 
        address: String::from("123 Main St"),
    };

    let Person { name, ..} = person;
    println!("Name: {}", name);

    // Underscore prefix for unused variables. 
    let _unused = 42; // No warning about unused variable. 

    //------------------------
    // References Patterns. 
    //------------------------
    // Match and work with references. 
    let reference = &4;
    match reference {
        &val => println!("Got value: {}", val),
    }
    // ref keyword to create reference in pattern. 
    let value = 5;
    match value {
        ref r => println!("Got reference to {}", r),
    }

    // ref mut for mutable reference. 
    let mut value = 5;
    match value {
        ref mut r => {
            *r += 1;
            println!("Modified to {}", r);
        }
    }

    // In struct patterns. 
    struct Data {
        value: String, 
    }
    let data = Data {
        value: String::from("hello")
    }
    match data {
        Data { ref value } => {
            println!("Borrowed: {}", value);
        }
    }
    // data is still valid here because we only borrowed value. 

    //---------------------------------
    // Patterns in Function Parameters
    //---------------------------------

    // With closures. 
    let points = vec![(0, 0), (1, 1), (2, 2)];
    let sum: i32 = points
                    .iter()
                    .map(|(x, y)| x + y)
                    .sum();
    println!("Sum: {}", sum);
    print_coordinates(&(3, 5));
}


