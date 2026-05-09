use std::{fs::read, io::Read};

fn echo<T>(value: T) -> T {
    value
    // returning the value of Type T.
}

fn echo_i32(value: i32) -> i32 {
    value
}

fn echo_bool(value: bool) -> bool {
    value
}

#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn double_number(number: MyOption<i32>) -> MyOption<i32> {
    match number {
        MyOption::Some(value) => MyOption::Some(value * 2),
        MyOption::None => MyOption::None,
    }
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

type MyError = &'static str;

fn safe_divide(a: i32, b: i32) -> MyResult<i32, MyError> {
    if b == 0 {
        MyResult::Err("Should not divide by zero")
    } else {
        MyResult::Ok(a / b)
    }
}

fn read_file_contents(path: &str) -> MyResult<String, std::io::Error> {
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return MyResult::Err(e),
    };

    let mut contents = String::new();

    if let Err(e) = file.read_to_string(&mut contents) {
        return MyResult::Err(e);
    }

    MyResult::Ok(contents)
}

fn main() {
    // println!("{}", echo_i32(5));
    // println!("{}", echo_bool(true));
    println!("{}", echo(5));
    println!("{}", echo(true));
    println!("{}", echo(String::from("Hello World")));

    let maybe_number = Some(42);
    println!("Unwrap: {}", maybe_number.unwrap());
    println!("Expect: {}", maybe_number.expect("value must exist"));

    // unwrap -> no argument.
    // expect -> takes some argument.
    // let maybe_number1: Option<i32> = None;
    // println!("Expect: {}", maybe_number1.unwrap());

    let books = ["The Little Prince", "1984", "Harry Potter"];

    let third_books = books.get(2);
    println!("Third Book is: {third_books:?}");

    let number1 = MyOption::Some(5);
    let number2: MyOption<i32> = MyOption::None;

    println!("Double Some(5): {:?}", double_number(number1));
    println!("Double None: {:?}", double_number(number2));

    let a: MyResult<i32, MyError> = MyResult::Ok(42);
    let b: MyResult<i32, MyError> = MyResult::Err("Something went wrong");

    println!("a = {a:?}");
    println!("b = {b:?}");

    let good_result = safe_divide(10, 4);
    let bad_result = safe_divide(10, 0);

    println!("10 / 4 = {good_result:?}");
    println!("10 / 0 = {bad_result:?}");

    // Working with file.
    match read_file_contents("my_content.txt") {
        MyResult::Ok(text) => println!("File Content is: {text}"),
        MyResult::Err(e) => println!("Error reading the contents: {e}"),
    }
}
