#[cfg(test)]
mod setup;

const SCORES: [Option<i32>; 6] = [Some(85), None, Some(92), Some(78), None, Some(95)];
const USER_INPUTS: [Option<&str>; 4] = [Some("42"), Some("hello"), None, Some("100")];

fn get_score_with_panic(scores: &[Option<i32>], index: usize) -> i32 {
    // TODO: Extract the score at the given index, panic if None or out of bounds
    0
}

fn get_score_with_message(scores: &[Option<i32>], index: usize) -> i32 {
    // TODO: Extract the score at the given index with a custom panic message
    0
}

fn get_score_or_default(scores: &[Option<i32>], index: usize, default: i32) -> i32 {
    // TODO: Extract the score or return the default value if None
    0
}

fn parse_number_or_panic(input: Option<&str>) -> i32 {
    // TODO: Parse the string to number, panic with message if None or invalid
    0
}

fn main() {
    println!("=== Option Unwrap and Expect Practice ===");

    // Test your implementations
    println!("Score at index 0: {}", get_score_with_panic(&SCORES, 0));
    println!("Score at index 2: {}", get_score_with_message(&SCORES, 2));
    println!(
        "Score at index 1 (or 0): {}",
        get_score_or_default(&SCORES, 1, 0)
    );
    println!("Parsed number: {}", parse_number_or_panic(USER_INPUTS[0]));

    // Uncomment to test panic behavior:
    // println!("This will panic: {}", get_score_with_panic(&SCORES, 1));
    // println!("This will panic: {}", parse_number_or_panic(USER_INPUTS[1]));
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// get_score_with_panic:
//   - Access scores[index] then call .unwrap() on the Option
//   - Look up: Option::unwrap(), slice indexing
//
// get_score_with_message:
//   - Use .expect("your message here") instead of unwrap
//   - Look up: Option::expect()
//
// get_score_or_default:
//   - Use .unwrap_or(default) on the Option
//   - Look up: Option::unwrap_or()
//
// parse_number_or_panic:
//   - First unwrap the outer Option, then parse the &str to i32
//   - Look up: str::parse(), expect()
