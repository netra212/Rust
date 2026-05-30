#[cfg(test)]
mod setup;

#[derive(Debug, PartialEq)]
enum MyOption<T> {
    Some(T),
    None,
}

fn create_some_value(value: i32) -> MyOption<i32> {
    // TODO: Create a MyOption::Some containing the given value
    MyOption::None
}

fn double_if_some(option: MyOption<i32>) -> MyOption<i32> {
    // TODO: Double the value if Some, return None if None
    MyOption::None
}

fn combine_options(opt1: MyOption<i32>, opt2: MyOption<i32>) -> MyOption<i32> {
    // TODO: Add the values if both are Some, return None otherwise
    MyOption::None
}

fn convert_to_string(option: MyOption<i32>) -> MyOption<String> {
    // TODO: Convert the integer to string if Some, preserve None
    MyOption::None
}

fn main() {
    println!("=== Custom Option Enum Practice ===");

    // Test your implementations
    let some_5 = create_some_value(5);
    println!("Created: {:?}", some_5);

    let doubled = double_if_some(some_5);
    println!("Doubled: {:?}", doubled);

    let none_val = MyOption::None;
    let doubled_none = double_if_some(none_val);
    println!("Doubled None: {:?}", doubled_none);

    let combined = combine_options(create_some_value(3), create_some_value(7));
    println!("Combined 3 + 7: {:?}", combined);

    let combined_none = combine_options(create_some_value(3), MyOption::None);
    println!("Combined 3 + None: {:?}", combined_none);

    let as_string = convert_to_string(create_some_value(42));
    println!("As string: {:?}", as_string);
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// create_some_value:
//   - Return MyOption::Some(value)
//   - Look up: enum variant construction with data
//
// double_if_some:
//   - Match on the enum: MyOption::Some(v) => MyOption::Some(v * 2), MyOption::None => MyOption::None
//   - Look up: match expressions, destructuring enum data
//
// combine_options:
//   - Match on both options simultaneously: (MyOption::Some(a), MyOption::Some(b)) => ...
//   - Look up: tuple patterns in match
//
// convert_to_string:
//   - Match Some(v) => MyOption::Some(v.to_string()), None => MyOption::None
//   - Look up: to_string(), type conversion inside match
