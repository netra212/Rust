// Closures -> Allow to define anonymous functions and capture variables from their surrounding environment.

// Closure system is defined by three core traits - Fn, FnMut, & FnOnce - that determine how closures interact with captured variables, how many times they can be called, and whether they can modify their environment.

// FnOnce: Can be called once.
// FnMul: Can be called multiple times. and can modify captured variables when invoked.
// Fn: can called multiple times, and only reads captured variables without modifying them.

// There is a inheritance relationships among three traits.

// Fn inherits from FnMut, and FnMut inherits from FnOnce.
fn main() {
    // Add
    let add = |x: i32, y: i32| -> i32 {
        return x + y;
    };
    let result = add(2, 4);
    println!("2 + 4 : {}", result);

    // Multiply
    let multiply = |x: i32, y: i32| -> i32 {
        return x * y;
    };
    let result1 = multiply(2, 4);
    println!("2 * 4 : {}", result1);

    // Closure.
    let numbers = vec![1, 2, 3, 4, 5];
    let print_number = || println!("Before Closure: {numbers:?}");
    print_number();
    print_number();
    print_number();
    println!("After Closure: {numbers:?}");

    let mut numbers1 = vec![1, 2, 3, 4, 5];
    println!("Before Closure: {numbers1:?}");
    let mut add_number = || numbers1.push(100);
    add_number();
    add_number();
    add_number();
    println!("After Closure: {numbers1:?}");

    let heap_string = String::from("heap string");
    let capture_string = || heap_string;
}
