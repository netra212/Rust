fn main() {
    println!("Hello from the main function!");

    // Calling another function
    another_function();

    print_value(5); // Pass the value 5
    print_sum(10, 20); // Pass two values

    let five = give_me_five();
    println!("The function returned: {}", five);

    let sum = add(10, 20);
    println!("The sum is: {}", sum);

    let sum = add(10, 20);
    println!("The sum is: {}", sum);

    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // You can also use `else if` for multiple conditions
    let number2 = 6;
    if number2 % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is not divisible by 4 or 3");
    }
}

// This function returns a value of type i32
fn give_me_five() -> i32 {
    5 // No semicolon means this is the return value
}

// This function takes two i32s and returns an i32
fn add(x: i32, y: i32) -> i32 {
    x + y // The result of this expression is returned
}

// Defining another function
fn another_function() {
    println!("Hello from another function!");
}

fn print_value(x: i32) {
    // `x` is a parameter of type i32
    println!("The value is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}
