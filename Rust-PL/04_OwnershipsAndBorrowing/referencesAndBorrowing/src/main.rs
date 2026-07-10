fn main() {
    println!("------- References and Borrowing -------");

    let s1 = String::from("Netra Khatri");
    let len = calculate_len(&s1); // Passing the reference of s1 not actually s1.
                                  // NOTE: &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because the reference does not own it, the value it points to will not be dropped when the references stops being used.
    println!("The length of string '{s1}' is {len}");

    //mutable reference example.
    let mut s = String::from("hello");
    change(&mut s);
    println!("The length of mutable reference string is 's' is: {s}");

    // NOTE: mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This below code that attempts to create two mutable references to s will fail:
    // let mut a = String::from("hello");
    // let a1 = &mut a;
    // let b2 = &mut a;
    // println!("{r1}, {r2}");

    // This above process will stops rust from being data races at compile time.
    /**
     * A data race is similar to a race condition and happens when these three behaviour occur:
     * 1.) Two or more pointers access the same data at the same time.
     * 2.) At least one of the pointer is being used to write to the data.
     * 3.) There's no mechanism being used to synchronize access to the data.
     */
    // Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // NOTE: We cannot have a mutable reference while we have an immutable one to the same value.

    /**
     * Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references is in the println!, before the mutable reference is introduced:
     *
     */
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed: The compiler can tell that the reference is no longer being used at a point before the end of the scope.

    // Dangling References
    // dangling pointer - a pointer that references a location in memory that may have been given to someone else - by freeing some memory while preserving a pointer to that memory. In Rust, By contrast, the compiler guarantees that references will never be dangling references: If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the references to the data does.

    let reference_to_string = dangle();
}

fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away. Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String.

// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // This works without any problems. Ownership is moved out, and nothing is deallocated.

fn calculate_len(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownerships of what it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", Lidl Ltd.");
}
// The rule of references.
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.
