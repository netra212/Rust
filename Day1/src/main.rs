fn main() {
    // In Rust, By default every variables is immutable.
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
}
