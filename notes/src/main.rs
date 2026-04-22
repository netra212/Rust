fn main() {
    println!("Hello, world!");

    let mut message = String::from("Hello"); // I INTEND to change this later
    println!("{}", message); // Prints "Hello"

    message = String::from("World"); // This is now allowed
    println!("{}", message); // Prints "World"

    // Integers (i8, u8, i32, u32, i64, u64, isize, usize)
    let a: i32 = -10; // 'i' for signed, 'u' for unsigned
    let b: u64 = 100;

    // Floating-point numbers
    let c: f64 = 3.14; // f32 is also available

    // Boolean
    let d: bool = true;

    // Character (a single Unicode scalar value)
    let e: char = '🦀';

    let x = 5;
    let y = x; // A copy of 5 is made and assigned to y.

    println!("x = {}, y = {}", x, y); // Both x and y are valid and can be used.

    let my_string = String::from("Hi 🦀");

    // This creates a String object on the stack, which in turn
    // allocates memory on the heap to store "Hello, world!".
    let s = String::from("Hello, world!");

    // Tuple: A simple way to group a fixed number of values with a variety of types.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    println!("The value of y is: {}", y); // Prints 6.4
    println!("The second value is: {}", tup.1); // Access by index

    // Question 1: "What happens if I don't mention the type?" (Type Inference)

    // In many of our examples, we wrote let x = 5; instead of let x: i32 = 5;. How does Rust know what type x is?

    // This feature is called **Type Inference**. The Rust compiler is extremely smart. It will look at the value you've provided and the context in which you use it, and it will infer the most logical type.

    let num = 5; // You didn't specify a type.
                 // The compiler thinks: "5" is a number. By default, I'll assume
                 // the programmer wants a standard 32-bit signed integer.
                 // So, I will infer the type of `num` to be `i32`.

    let pi = 3.14; // You didn't specify a type.
                   // The compiler thinks: "3.14" has a decimal point. By default,
                   // I'll assume the programmer wants a standard 64-bit float.
                   // So, I will infer the type of `pi` to be `f64`.

    // This code will NOT compile!
    // let guess = "42".parse().expect("Not a number!");
    //

    // -------------- The Three Rules of Ownership --------------
    // Rule 1: Each value in Rust has a variable that’s called its owner.
    // When this line executes, memory is allocated on the heap for the text "hello".
    // The variable `s1` is created on the stack.
    // `s1` is now the OWNER of that heap data.
    let s1 = String::from("hello");

    // Rule 2: There can only be one owner at a time.

    // let s1 = String::from("hello"); // s1 owns the "hello" data on the heap.

    // Here is the key moment:
    // let s2 = s1; // What happens here?

    // Now, let's try to use s1 again.
    // This line will NOT compile!
    // println!("s1 is: {}", s1);

    // Rule 3: When the owner goes out of scope, the value will be dropped
    {
        // s is not valid here, it’s not yet declared
        let s = String::from("hello"); // s is valid from this point forward
                                       // ... do stuff with s
    } // This scope is now over. `s` is the owner of the heap data.
      // Because the owner is going out of scope, Rust automatically calls `drop` on `s`.
      // The memory for "hello" is freed from the heap.
      // s is no longer valid from this point forward.
}
