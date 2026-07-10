fn main() {
    println!("------ SLICE TYPE ------");
    /**
     * Slices let you reference a contiguous sequence of elements in a collection. A slice is a kind of reference, so it does not have ownership.
     */
    // String Slices
    // A string is a reference to a contiguous sequence of the elements of a String, and it looks like this:
    let s = String::from("hello world.");
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s);
    s.clear();
    // above will throw error!
    // println!("The first word is: {word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Converting string into an array of bytes so s.as_bytes() method used.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {}
