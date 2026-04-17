fn main() {
    let my_string = String::from("Hello, World!");
    println!("{my_string}");

    takes_ownership(my_string);
    // my_string is no longer valid here. You cannot use it.

    let x = 5;
    makes_copy(x);
    println!("{x}"); // x is still valid (i32 implements Copy)

    // &str (string slice - immutable, stored in binary)
    let s1: &str = "Hello, World";
    println!("{s1}");

    // String (heap-allocated, mutable)
    let mut s2 = String::from("My String");
    println!("{s2}");
    s2.push_str(", world");
    println!("{s2}");

    let s3 = &s2;
    println!("I borrowed string from s2: {s3}");

    let s4 = s1.to_string();
    println!("{s4}");

    let original = String::from("hello");
    let clone = original.clone();

    println!("original: {original}");
    println!("original on the stack address, {:p}", &original);
    println!("original on the heap, {:p}", original.as_ptr());

    println!();
    println!("clone: {clone}");
    println!("original on the stack address, {:p}", &original);
    println!("original on the heap, {:p}", original.as_ptr());

    let king = String::from("Ruler 1");
    claim_throne(&king);
    println!("Current King is: {king}");

    let ruler_2 = String::from("Ruler 2");
    let mut ruler_2 = pass_throne(ruler_2);
    println!("New ruler of empire is: {ruler_2}");

    /*
    Borrowing in Rust:
    Borrowing allows references to a value without taking ownerships, enabling multiple parts of the code to access the same data without violating Rust's ownerships rules.

    Rules:
        1. Mulable (read and write) and immutable (read-only) borrowing may not be made to the same value, at the same time.
        2. A value can either have a single mutiple borrow or unlimited immutable borrows.
        3. A borrowed reference can not outlive it's scope or the original one.
    */
    let mut data = String::from("hello");
    let ib1 = &data;
    let ib2 = &data;

    // let mb3 = &mut data;
    // NOTE: mutable and immutable may not be made to the same value, at the same time.
    println!("ib1: {ib1}, ib2: {ib2}");

    // Defining the scope.
    {
        let ib1 = &data;
        let ib2 = &data;
        println!("ib1: {ib1}, ib2: {ib2}");
    } // dropped.

    let mb3 = &mut data;
    mb3.push_str(", world function..!");

    println!("mb3: {mb3}");
}

fn pass_throne(name: String) -> String {
    println!("{name} is passing the Throne");
    name
}

fn claim_throne(name: &String) {
    println!("{name} is claiming the Throne.");
} // value will be dropped

fn takes_ownership(string_value: String) {
    println!("Takes ownership: {string_value}");
} // string_value dropped here

fn makes_copy(some_integer: i32) {
    // Changed name to match usage
    println!("Value is : {some_integer}");
}
