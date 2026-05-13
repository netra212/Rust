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

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // The value assigned to `number` will be the result of the `if` expression.

    println!("The number is: {}", number); // Prints 5

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("Again!");

        if counter == 10 {
            break counter * 2; // `break` can also return a value from the loop
        }
    };

    println!("The result of the loop is: {}", result); // Prints 20

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50]; // This is an array

    for element in a {
        // `for` loop iterates through each element of `a`
        println!("the value is: {}", element);
    }

    // `1..4` creates a range from 1 up to (but not including) 4.
    // The `.rev()` reverses the range.
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // The type of `a` is `[i32; 5]` - an array of 5 elements of type i32.
    // The compiler can usually infer this for you.
    let a = [1, 2, 3, 4, 5];

    // Create an array where all elements are the same value.
    // `[3; 5]` means "an array of 5 elements, all with the value 3".
    let b = [3; 5]; // This is `[3, 3, 3, 3, 3]`

    // Accessing elements by index (starts from 0)
    let first = a[0];
    let second = a[1];

    println!("The first element of a is: {}", first);
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
