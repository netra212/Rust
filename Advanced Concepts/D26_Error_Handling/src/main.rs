/**
 * Two main Types of Errors:
 * 1. Recoverable Errors: Result<T, E> eg: File not found, invalid input.
 * 2. Unrecoverable Errors: panic!  Array index out of bounds.
 *
 * NOTE: In rust, there is no any exceptions like other programming languages.
 *
 * Best Practices:
 * -> Prefer Result for recoverable errors.
 * -> Use panic! only for unrecoverable situations.
 * -> Add context to errors using crates like thiserror.
 */
use std::fs::File;
use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("h1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
