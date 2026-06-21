use anyhow::{Context, Result};
use std::fmt::{self, format};
use thiserror::Error;

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

#[derive(Debug)]
enum NetworkError {
    Timeout,
    ConnectionRefused,
    InvalidUrl,
}

impl fmt::Display for NetworkError {
    //
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::Timeout => write!(f, "Request is Timeout"),
            NetworkError::ConnectionRefused => write!(f, "Request is Connection Refused"),
            NetworkError::InvalidUrl => write!(f, "Request is Invalid Url"),
        }
    }
}

fn connect(url: &str) -> Result<String, NetworkError> {
    if !url.starts_with("http") {
        return Err(NetworkError::InvalidUrl);
    }

    if url.contains("slow") {
        return Err(NetworkError::Timeout);
    }

    if url.contains("blocked") {
        return Err(NetworkError::ConnectionRefused);
    }

    Ok("Connected!".to_string())
}

fn parse_age_without(input: &str) -> Result<i32, String> {
    let age: i32 = input
        .parse::<i32>()
        .map_err(|err| format!("Failed to parse: {}", err))?;

    if age < 0 || age > 150 {
        return Err("Age must be between 0 & 150".to_string());
    }

    Ok(age)
}

fn parse_age_with_anyhow(input: &str) -> Result<i32> {
    let age = input.parse::<i32>().context("Failed to parse");

    if age < 0 || age > 150 {
        anyhow::bail!("Age must be between 0 & 150");
    }

    Ok(age)
}

#[derive(Error, Debug)]
enum ProductError {
    #[error("Product '{0}' is out of stock")]
    OutOfStock(String),
    #[error("Product '{0}' is to expensive: ${1}")]
    TooExpensive(String, f64),
}

fn validate_product(name: &str, price: f64) -> Result<String, ProductError> {
    if name.is_empty() {
        return Err(ProductError::OutOfStock(name.to_string()));
    }

    if price > 5000.0 {
        return Err(ProductError::TooExpensive(name.to_string(), price));
    }

    Ok(format!("Product: '{}' is available at ${:.2}", name, price))
}

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("h1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| "Invalid".to_string())
}

fn double(n: i32) -> Result<i32, String> {
    Ok(n * 2)
}

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    match connect("www.example.com") {
        Ok(msg) => println!("Ok {}", msg),
        Err(err) => println!("Err {:?}", err),
    }

    match connect("https://slow-connection.com") {
        Ok(msg) => println!("Ok {}", msg),
        Err(err) => println!("Err {:?}", err),
    }

    match connect("https://blocked-website.com") {
        Ok(msg) => println!("Ok {}", msg),
        Err(err) => println!("Err {:?}", err),
    }

    match connect("https://good-website.com") {
        Ok(msg) => println!("Ok {}", msg),
        Err(err) => println!("Err {:?}", err),
    }

    println!("========= WITHOUT ANYHOW =========");
    print!("Valid: {:?}", parse_age_without("30"));
    print!("Invalid: {:?}", parse_age_without("abc"));
    print!("Out of ranges: {:?}", parse_age_without("1300"));

    println!("========= WITH ANYHOW =========");
    print!("Valid: {:?}", parse_age_with_anyhow("30"));
    print!("Invalid: {:?}", parse_age_with_anyhow("abc"));
    print!("Out of ranges: {:?}", parse_age_with_anyhow("1300"));

    match validate_product("", 100.0) {
        Ok(msg) => println!("Ok: {}", msg),
        Err(err) => println!("Err: {}", err),
    }

    match validate_product("Luxury Watch", 1000000.0) {
        Ok(msg) => println!("Ok: {}", msg),
        Err(err) => println!("Err: {}", err),
    }

    // chaining success.
    let result = parse("5").and_then(double);
    println!("{:?}", result);

    let result1 = parse("abc").and_then(double);
    println!("{:?}", result1);

    // Fallback.
    let result2 = parse("xyz").or_else(|_| Ok::<i32, String>(0));
    println!("{:?}", result2);
}
