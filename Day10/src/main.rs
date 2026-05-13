fn main() {
    println!("LIFETIME IN RUST");

    let reference = String::from("The reference.");
    let reference = &reference;

    println!("{reference}");

    // Variables scopes and Lifetimes.

    // Lifetimes for Variables.
    let r;
    {
        let x = 5;
        r = &x;
    }

    // println!("r is {r}");

    // A dangling reference is a reference to data that no longer exists.

    let languages = vec!["Rust", "JS", "Go", "Python"];
    let favorite_language = &languages[1..3];
    println!("Favorite Language: {:?}", favorite_language);

    // Generic Lifetimes in Rust are placeholders that tell the compiler how long reference live in relation to each other.

    // Example: 'a is a generic lifetime used to say << this reference must live at least as long as 'a >>.
}

fn get_first<'a>(text: &'a str) -> &'a str {
    &text[0..1]
}
