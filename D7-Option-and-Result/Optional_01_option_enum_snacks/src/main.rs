#[cfg(test)]
mod setup;

// Custom Option enum
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

// Function to check if a snack is available
fn get_snack(snack_name: &str, machine: [&str; 3]) -> MyOption<String> {
    for &snack in &machine {
        if snack == snack_name {
            return MyOption::Some(snack.to_string());
        }
    }
    MyOption::None
}

fn main() {
    let machine = ["chips", "cookie", "water"];

    let favorite_snack = "cookie";
    let result = get_snack(favorite_snack, machine);

    match result {
        MyOption::Some(snack) => println!("🎉 Snack found: {}", snack),
        MyOption::None => println!("😢 No {} in the machine.", favorite_snack),
    }

    // Try searching for something not there
    let result2 = get_snack("soda", machine);

    match result2 {
        MyOption::Some(snack) => println!("🎉 Snack found: {}", snack),
        MyOption::None => println!("😢 No soda in the machine."),
    }
}
