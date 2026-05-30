#[cfg(test)]
mod setup;

#[derive(Debug, PartialEq)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn create_success(value: i32) -> MyResult<i32, String> {
    // TODO: Create a successful result containing the given value
    MyResult::Err("placeholder".to_string())
}

fn validate_age(age: i32) -> MyResult<i32, String> {
    // TODO: Validate age is between 0 and 120, return appropriate error message
    MyResult::Err("placeholder".to_string())
}

fn parse_number(input: &str) -> MyResult<i32, String> {
    // TODO: Parse string to number, return error if invalid
    MyResult::Err("placeholder".to_string())
}

fn divide_numbers(a: i32, b: i32) -> MyResult<i32, String> {
    // TODO: Divide a by b, return error if b is zero
    MyResult::Err("placeholder".to_string())
}

fn main() {
    println!("=== Custom Result Enum Practice ===");

    // Test your implementations
    println!("Success(42): {:?}", create_success(42));

    println!("Valid age 25: {:?}", validate_age(25));
    println!("Invalid age -5: {:?}", validate_age(-5));
    println!("Invalid age 150: {:?}", validate_age(150));

    println!("Parse '123': {:?}", parse_number("123"));
    println!("Parse 'abc': {:?}", parse_number("abc"));

    println!("Divide 10/2: {:?}", divide_numbers(10, 2));
    println!("Divide 10/0: {:?}", divide_numbers(10, 0));
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// create_success:
//   - Return MyResult::Ok(value)
//   - Look up: enum variant construction
//
// validate_age:
//   - Check if age is in the valid range (0..=120); return Ok or Err with a message
//   - Look up: if/else, range checks, format!()
//
// parse_number:
//   - Try parsing input.parse::<i32>(); map Ok/Err to MyResult variants
//   - Look up: str::parse(), match on Result, map_err
//
// divide_numbers:
//   - Check if b == 0 and return an error; otherwise return MyResult::Ok(a / b)
//   - Look up: conditional return, division
