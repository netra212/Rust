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

    // While loop.
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

    let animals = ["cat", "dog", "elephant"];
    let mut index = 0;
    loop {
        println!("Animals is: {}", animals[index]);
        // Incrementing the value of index.
        index += 1;

        // stopping condition of loop.
        if index >= animals.len() {
            break;
        }
    }

    // Exercise:-
    let a: i8 = 120; // By default it takes 8 bytes of memory. i -> means signed integer. 
    let b: u16 = 20; // u -> unsigned means non-negative integers.
    let c: f32 = 12.19;
    let d: bool = true;
    let ch: char = 'n';
    println!("a={}, b={}, c={}, d={}, ch={}", a, b, c, d, ch);

    // Primitive Data types.
    // Number, Floating, Boolean, Character, tuples,arrays.

    let mut str1 = String::from("Netra Khatri");
    str1.push_str("Hello");
    println!("{} {}", str1.len(), str1.capacity());

    let p = 90;
    let d = p;
    println!("{} {}", d, p);
    let ans = add_number(3, 4);
    println!("{}", ans);

    // let str2 = String::from("Hello");
    // finder(str2);

    // let mut strike = String::from("Strike is Comming");

    // // Only read the data.
    // immutable_borrow(&strike);

    // // mutable_borrowing (Have both Read and Write Permission).
    // mutable_borrow(&mut strike);

    // let r2 = &mut strike;
    // let r1 = &strike;

    // println!("{} {}", r1, r2);
    let arr = [10, 20, 30, 40, 50, 60];
    // println!("{}", arr[0]);
    for element in arr {
        println!("{}", element);
    }

    for num in 1..=5 {
        print!("{}", num);
    }

    // Day 4: Rust Learning.
    println!("\n\n------ DAY - 4 ------\n");
    let a1 = "Confused String"; // a1 will be treated as a static variable and will not store in either stack or heap. 
    // a1 = "Confused String"; will not be store in stack because in stack, we store only fixed data size but this is not fixed data size. In stack, we cannot perform any operations like push, pop and so on.
    // In stack, we cannot store a1 = Confused String";
    println!("\n{}", a1);

    println!("\n\n------- Vector (Non-Primitive Type) in Rust -------");
    // let mut arr: Vec<i32> = Vec::new();
    let mut arr = vec![10, 20, 30];
    arr.push(40);
    println!("{:?}", arr);
    println!("Length of Vector is: {}", arr.len());
    println!("Capacity of Vector is: {}", arr.capacity());
    println!("Pointer of Vector in Heap: {:p}", arr.as_ptr());
    println!("Address of Vector in stack: {:p}", &arr);
    println!("{:p}", &arr[0]); // Gives the Location of first data in heap.
    println!("{:p}", &arr[1]); // Gives the Location of second data in heap.
}

// fn mutable_borrow(s: &mut String) {
//     println!("\nMutable");
//     s.push_str(" --> Hi");
//     println!("{}", s);
// }
// fn immutable_borrow(s: &String) {
//     print!("{}", s);
// }

// fn finder(s: String) {
//     println!("{}", s);
//     // rust inject some sort of code here, similar to delete keyword in c++, which drops the function.
// }

fn add_number(x: i32, y: i32) -> i32 {
    return x + y;
}
