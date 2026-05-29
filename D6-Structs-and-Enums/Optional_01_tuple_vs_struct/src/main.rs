#[cfg(test)]
mod setup;

// * Tuple -> order matter
// * Struct -> order doesn't matter (name fields)

// Tuple version: (volume_ml, toxicity_level, color_code)
fn brew_with_tuple(potion: (u32, u8, &str)) {
    println!(
        "Brewing potion: {}ml, toxicity {}, color {}",
        potion.0, potion.1, potion.2
    );

    // Danger: swap volume and toxicity level!
    if potion.1 > 5 {
        println!("⚠️ Warning: Toxic potion!");
    }
}

// Struct version
struct Potion {
    volume_ml: u32,
    toxicity_level: u8,
    color_code: String,
}

fn brew_with_struct(potion: Potion) {
    println!(
        "Brewing potion: {}ml, toxicity {}, color {}",
        potion.volume_ml, potion.toxicity_level, potion.color_code
    );

    if potion.toxicity_level > 5 {
        println!("⚠️ Warning: Toxic potion!");
    }
}

fn main() {
    let tuple_potion = (250, 7, "red");
    brew_with_tuple(tuple_potion);

    // Mistake: swap values, still compiles!
    let bad_potion = (7, 250, "red");
    brew_with_tuple(bad_potion); // Output will be misleading!

    let safe_potion = Potion {
        volume_ml: 250,
        toxicity_level: 7,
        color_code: String::from("red"),
    };
    brew_with_struct(safe_potion);

    // Uncomment to test compiler catching errors:
    // let wrong_struct = Potion { toxicity_level: 7, volume: 250, color: "red" };
}
