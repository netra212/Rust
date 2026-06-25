use std::collections::HashMap;

// Define a macro.
macro_rules! say_hello {
    // Pattern matching: () means no arguments.
    () => {
        // What the macro expands to
        println!("Hello!")
    }; // Pattern matching: ($name: expr) means one expression
    ($name: expr) => {
        println!("Hello, {}!", $name)
    };
}

macro_rules! my_vec{
    ($($element: expr), *) => {{
        let mut v = Vec::new();
        $(v.push($element);)*
        v
    }};
}

macro_rules! hashmap {
    ($($key: expr => $value: expr), *) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

macro_rules! select {
    ($table: expr) => {
        format!("SELECT * FROM {}", $table)
    };
    ($table: expr, where $condition: expr) => {
        format!("SELECT * FROM {} WHERE {}", $table, $condition)
    };
}

fn main() {
    println!("Simple Macro Examples\n");

    // Before expansion:
    say_hello!();
    say_hello!("Alice");

    println!("\n// After expansion (what the compiler actually sees):");
    println!("// println!(\"Hello!\");");
    println!("// println!(\"Hello, {{}}\", \"Alice\");");

    //
    let numbers = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // hashmap.
    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 85,
        "Carol" => 92
    };
    println!("{:?}", scores);

    //
    let query1 = select!("users");
    let query2 = select!("users", where "age > 18");

    println!("{}", query1); // SELECT * FROM users
    println!("{}", query2); // SELECT * FROM users WHERE age > 18.
}
