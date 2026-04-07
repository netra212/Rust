const SECOND_IN_MINUTES: u8 = 60;

// Function in Rust.
fn add_explicit(x: i32, y: i32) -> i32 {
    return x + y;
}

fn add_idiomatic(x: i32, y: i32) -> i32 {
    x + y
}

// Since, We are not returning any thing fro the below function so the default return type will be unit type and return tuple like this ().
fn say_hello() {
    println!("hello");
}

// fn say_hello_1(name: &str) -> String {
//     format!("Hello, {}!", name);

fn main() {
    // In Rust, By default every variables is immutable.

    // Start of Inner block (scope).
    let name = "maxime";
    let age: u8 = 31;
    let language = "Rust";

    print!("Name: {name}, age: {age} && language: {language}");

    let rust = "Rust";
    let javascript = "JS";
    print!("I like {0} and {1}, but i prefer {0}", rust, javascript);

    // Variable shadowning.
    let number = 5;
    println!("Value of a Number is: {number}");
    let number = number + 1;
    println!("New value of number is: {number}");

    let number = number * 2;
    println!("New New value of a number is: {number}");

    let namepok = "Pokemon";
    let mut level = 5; // Now value can be mutable means can be changed. 
    println!("Name of Pokemon: {namepok} and Level is: {level}");

    // Level of Pokemon has been increased so.
    level = level + 5;
    println!("Name of Pokemon: {namepok} and new Level is: {level}");

    // Const in Rust.
    let mut elapsed = 0;

    println!("Start:  {elapsed} seconds");

    elapsed += SECOND_IN_MINUTES;

    println!("After 1 minutes: {elapsed} seconds");

    let x = 5;
    println!("Outer x = {x}");

    {
        // start of the scope.
        let y1 = 20;
        let x1 = 99;

        println!("Inner x = {x1}, Inner y = {y1}");

        // End of the scope, y is dropped.
    }

    let sum_of_two_numbers = add_explicit(2, 3);
    print!("sum of two number is: {sum_of_two_numbers}");

    let sum_of_two_numbers_idiomatic = add_idiomatic(2, 5);
    print!("sum of two number is: {sum_of_two_numbers_idiomatic}");

    let c = say_hello();
    println!("c: {:?}", c);

    // Arrays in Rust.
    let names = ["yagya", "radhika", "netra"];
    println!("\nNames :{}", names[0]);
    println!("Names :{}", names[1]);
    println!("Names :{}", names[2]);

    // len.
    println!("Total: {}", names.len());

    // Destructuring data from a tuple.
    let person_details = ("Alice", 25, "Software Engineer", 5.5);

    // Access the data from the tuple using index.
    println!("\nName: {}", person_details.0);
    println!("Age: {}", person_details.1);
    println!("Profession: {}", person_details.2);
    println!("Height: {}", person_details.3);

    let (name, profession, age, height) = person_details;
    println!("After Destructuring");
    println!("\nname: {name}, profession: {profession}, age: {age}, and height: {height}");
}

// Scopes and Blocks.
// A block is the area between an opening curly brace and closing curly brace.
