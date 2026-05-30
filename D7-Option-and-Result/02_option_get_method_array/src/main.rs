#[cfg(test)]
mod setup;

const SPORTS: [&str; 5] = ["Soccer", "Basketball", "Tennis", "Swimming", "Running"];
const SCORES: [i32; 4] = [85, 92, 78, 95];
const NAMES: [&str; 6] = ["Alice", "Bob", "Charlie", "Diana", "Eve", "Frank"];

fn safe_get_sport(sports: &[&str], index: usize) -> Option<String> {
    // TODO: Safely get a sport at the given index and return it as an owned String
    None
}

fn get_multiple_sports(sports: &[&str], indices: &[usize]) -> Vec<String> {
    // TODO: Get multiple sports by their indices, skip invalid indices
    vec![]
}

fn find_sport_position(sports: &[&str], target: &str) -> Option<usize> {
    // TODO: Find the index of a specific sport in the array
    None
}

fn get_score_range(scores: &[i32], start: usize, end: usize) -> Vec<i32> {
    // TODO: Safely get a range of scores from start to end (exclusive)
    vec![]
}

fn main() {
    println!("=== Safe Array Access Practice ===");

    // Test your implementations
    println!("Sport at index 2: {:?}", safe_get_sport(&SPORTS, 2));
    println!("Sport at index 10: {:?}", safe_get_sport(&SPORTS, 10));

    let indices = [0, 2, 4, 10];
    println!("Multiple sports: {:?}", get_multiple_sports(&SPORTS, &indices));

    println!("Position of Tennis: {:?}", find_sport_position(&SPORTS, "Tennis"));
    println!("Position of Golf: {:?}", find_sport_position(&SPORTS, "Golf"));

    println!("Scores 1-3: {:?}", get_score_range(&SCORES, 1, 4));
    println!("Scores 2-10: {:?}", get_score_range(&SCORES, 2, 10));
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// safe_get_sport:
//   - Use slice.get(index) which returns Option<&&str>, then map to owned String
//   - Look up: slice::get(), Option::map(), to_string()
//
// get_multiple_sports:
//   - Iterate over indices, call safe_get_sport, flatten/filter None
//   - Look up: iter(), filter_map(), flat_map()
//
// find_sport_position:
//   - Use iter().position() with a predicate that matches the target
//   - Look up: Iterator::position()
//
// get_score_range:
//   - Use get(start..end) on the slice; handle None (out of range) by returning empty vec
//   - Look up: slice::get() with ranges, unwrap_or_default()
