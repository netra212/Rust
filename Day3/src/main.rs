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
}

fn takes_ownership(string_value: String) {
    println!("Takes ownership: {string_value}");
} // string_value dropped here

fn makes_copy(some_integer: i32) {
    // Changed name to match usage
    println!("Value is : {some_integer}");
}
