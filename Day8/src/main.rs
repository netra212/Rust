#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    location: String,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            first_name: "John".into(),
            last_name: "Doe".into(),
            age: 23,
            location: "London".into(),
        }
    }
}

struct Dog {}

struct Cat {}

pub trait MakeSound {
    fn make_sound(&self);
}

impl MakeSound for Dog {
    fn make_sound(&self) {
        println!("Woof Woof");
    }
}

impl MakeSound for Cat {
    fn make_sound(&self) {
        println!("Meow Meow");
    }
}

fn print_as_string<T: ToString>(value: T)
where
    T: ToString,
{
    let s = value.to_string();
    println!("Value as a String: {}", s);
}

fn main() {
    // trait: similar to interface in other langauges.
    // A Trait is a contract that describes functionality that a type must have.
    println!("Traits and Implementations in Rust");

    let person = Person::default();
    println!("Person: {person:?}");

    let dog = Dog {};
    let cat = Cat {};

    dog.make_sound();
    cat.make_sound();

    print_as_string(42);
    print_as_string(4.2);
    print_as_string("Hello");
    print_as_string(String::from("Allocated String"));

    // let my_vec = vec!["Hello", "World"];
    // print_as_string(my_vec); // does not work because vec does not bound to string.
}
