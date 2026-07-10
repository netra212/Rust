fn main() {
    println!("\n----- OWNERSHIPS IN RUST -----\n");
    // /**
    //  * Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.
    //  *
    //  * ------ Ownership Rules ------
    //  * 1.) Each value in Rust has an owner.
    //  * 2.) There can only be one owner at a time.
    //  * 3.) When the owner goes out of scope, the value will be dropped.
    //  */
    let s = "hello world."; // this 's' variable is valid till the fn main() exit.

    {
        let x = "Hey from x"; // x scope starts.
    } // x scope ends.

    let mut s1: String = String::from("Hello World"); // this is the memory allocation by us.
    println!("S = {}", s1);
    s1.push_str("Netra khatri.");
    println!("S = {}", s1);

    let mut x = 5;
    let y = x;
    x = 10;
    println!("x = {}", x);
    println!("y = {}", y);

    let x1 = String::from("I am X");
    let y1 = x1; // when y1 = x1, which means y1 copies the pointer of x1 now both x1 and y1 both pointing on the same data on the heap, when y1 = x1 means data is not beingh copied here.
    // But when y1 and x1 both goes out of scope, both y1 and x1 will try to clean up the same memory. This is known as a double free error and is one of the memory safety bugs. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerablities.
    // So, How rust handle this problems ?
    // -> so, In order to ensure memory safety, after the line y1 = x1, rust consider that x1 as no longer valid. Therefore, rust does not need to free anything when x1 goes out of scope.
    //println!("x1 is: ", { x1 });

    let mut s1 = String::from("Hello netra");
    let s2 = s1.clone(); // This is an expensive operation. If we do want to deeply copy the heap data of the String, not just stack data, we can use a common method called clone. 
    // Now, I can push into s2.
    s2.push_str("This is s2");

    /**
     * In Rust, String is made up of three parts.
     * 1.) a pointer to the memory.
     * 2.) length.
     * 3.) capacity: total amount of memory in bytes, that the String has received from the allocator.
     */
    //
    let num = 10;
    let result = add(num);

    //
    let name = String::from("Netra Khatri...!");

    let s = gives_ownerships();

    let s2 = takes_and_gives_back(s);
    println!("s2 = ", { s2 });

    take_ownership(name);

    println!("Num is {num} and result = {result}");
    //   println!("Value of name is:",{name}); This will throw an error because after

    let s = String::from("Netrak");
    let (s1, len1) = calculate_len(s);
    println!("The len of {s1} is {len1}");
}

fn calculate_len(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}

// this function will returns the ownership.
fn gives_ownerships() -> String {
    let s = String::from("This is a string from gives ownerships");
    s
}

fn take_ownership(s: String) {
    println!("Insider Ownerships {s}");
}

fn add(x: i32) -> i32 {
    x + 10
}

fn takes_and_gives_back(s: String) -> String {
    println!("S in takes_and_gives_back: ", { s });
    s
}
