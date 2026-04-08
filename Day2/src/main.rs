// --------------- DAY 2: CONTROL FLOW --------------- //

fn main() {
    let number = 7;

    if number > 0 {
        println!("Number is Positive");
    } else {
        println!("Number is Negative");
    }

    let mut count = 0;
    loop {
        println!("Count is {count}");
        count += 1;

        if count == 3 {
            break;
        }
    }

    // while loop.
    let mut number1 = 3;
    println!("---- While Loop in Rust ---- ");
    while number1 > 0 {
        println!("Number is {number1}");
        number1 -= 1;
    }

    // For Loop in rust.
    for i in 1..=10 {
        println!("Index i is {i}");
    }

    // Match statement in Rust.
    let number2 = 2;
    match number2 {
        1 => println!("One "),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    let age = 20;
    let has_id = true;

    if age > 18 && has_id {
        println!("Eligible for voting");
    } else {
        println!("Not Eligible for voting");
    }

    // Exercise 1: LOOP.
    // Complete the TODOs in `src/main.rs` to fix the loop implementation:

    // 1. Replace the placeholder values with correct animal names
    // 2. Fix the index increment to move through the array
    // 3. Fix the break condition to stop when all animals are printed
}
