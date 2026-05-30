#[cfg(test)]
mod setup;

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn parse_user_name(input: &str) -> Result<String, String> {
    // TODO: Parse name from format "name:Alice", return error if invalid format
    Err("not implemented".to_string())
}

fn parse_user_age(input: &str) -> Result<u8, String> {
    // TODO: Parse age from format "age:25", return error if invalid format or number
    Err("not implemented".to_string())
}

fn parse_user_email(input: &str) -> Result<String, String> {
    // TODO: Parse email from format "email:alice@example.com", validate it contains @
    Err("not implemented".to_string())
}

fn parse_complete_user(name_input: &str, age_input: &str, email_input: &str) -> Result<User, String> {
    // TODO: Parse all three inputs and create a User, propagate any errors with ?
    Err("not implemented".to_string())
}

fn main() {
    println!("=== Result Question Mark Practice ===");

    // Test your implementations
    println!("Parse name: {:?}", parse_user_name("name:Alice"));
    println!("Parse invalid name: {:?}", parse_user_name("invalid"));

    println!("Parse age: {:?}", parse_user_age("age:25"));
    println!("Parse invalid age: {:?}", parse_user_age("age:abc"));

    println!("Parse email: {:?}", parse_user_email("email:alice@example.com"));
    println!("Parse invalid email: {:?}", parse_user_email("email:invalid"));

    let user_result = parse_complete_user("name:Bob", "age:30", "email:bob@test.com");
    println!("Complete user: {:?}", user_result);

    let invalid_user = parse_complete_user("invalid", "age:30", "email:bob@test.com");
    println!("Invalid user: {:?}", invalid_user);
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// parse_user_name / parse_user_age / parse_user_email:
//   - Split on ':' and check that the prefix matches the expected key
//   - For age: further parse the value part to a number
//   - For email: check that the value contains '@'
//   - Look up: str::split_once(), str::parse(), contains()
//
// parse_complete_user:
//   - Call each parse function and use ? to propagate errors
//   - Build the User struct once you have all three values
//   - Look up: ? operator, Result propagation, struct initialization
