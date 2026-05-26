#[derive(Default)]
struct Event {
    name: String,
    date: String,
    location: String,
}

#[derive(Debug, Default)]
struct Character {
    name: String,
    health: u32,
    power: u32,
}

struct Book {
    pub title: String,
    pub author: String,
    pub year: i32,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i32) -> Self {
        Book {
            title: title.into(),
            author: author.into(),
            year,
        }
    }

    pub fn open(&self) {
        println!(
            "The Book {} was written by {} in {}.",
            self.title, self.author, self.year
        );
    }
}

struct Color(u8, u8, u8);

struct User {
    pub username: String,
    age: u8,
}

impl User {
    pub fn new(username: String, age: u8) -> Self {
        User { username, age }
    }
}

mod user_module {
    pub struct User {
        pub username: String,
        age: u8,
    }

    impl User {
        pub fn new(username: String, age: u8) -> Self {
            User { username, age }
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }
    }
}

/**
 * enums is a custom data type that allows us to define a type by listing all possible values it can have.
 */

enum LightState {
    On,
    Off,
}

fn print_state(state: LightState) {
    match state {
        LightState::On => println!("The Light is ON."),
        LightState::Off => println!("The Light is OFF."),
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn print_direction(direction: Direction) {
    match direction {
        Direction::North => println!("The direction is NORTH."),
        Direction::South => println!("The direction is SOUTH."),
        Direction::East => println!("The direction is EAST."),
        Direction::West => println!("The direction is WEST."),
    }
}

enum IpAddressKind {
    ipV4,
    ipV6,
}

fn route(ip_kind: IpAddressKind) {
    match ip_kind {}
}

pub struct Movies {
    pub title: String,
    pub director: String,
    pub duration: i32,
    pub budget: i32,
    pub world_wide_collection: i32,
    pub rating: i32,
}

impl Movies {
    pub fn new(
        title: &str,
        _director: &str,
        duration: i32,
        budget: i32,
        world_wide_collection: i32,
        rating: i32,
    ) -> Self {
        Movies {
            title: title.to_string(),
            director: _director.to_string(),
            duration,
            budget,
            world_wide_collection,
            rating,
        }
    }

    pub fn print_movies_details(&self) {
        println!("Movies Details");
        println!()
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

struct Car {
    light_seen: TrafficLight,
}

impl Car {
    pub fn react_to_light(&self) {
        match self.light_seen {
            TrafficLight::Red => println!("Car Stops"),
            TrafficLight::Yellow => println!("Car Slow"),
            TrafficLight::Green => println!("Car Go"),
        }
    }
}

fn main() {
    //
    let event = Event {
        name: String::from("Rust Meetup."),
        date: String::from("11-05-2026"),
        location: String::from("London"),
    };

    println!("Event Location: {}", event.name);
    println!("Event Location: {}", event.date);
    println!("Event Location: {}", event.location);

    let default_character = Character::default();
    println!("Default: {default_character:?}");

    let hero = Character {
        name: String::from("SuperHero"),
        ..Default::default()
    };

    println!("Hero: {hero:?}");

    let ancient_book = Book::new("Title Ancient", "Ancient Book", 234);
    let modern_book = Book::new("Title Modern", "Modern Book", 2024);

    ancient_book.open();
    modern_book.open();

    let sky_blue = Color(135, 235, 60);
    println!("Red: {}", sky_blue.0);
    println!("Green: {}", sky_blue.1);
    println!("Blue: {}", sky_blue.2);

    let user = user_module::User::new(String::from("Alice"), 34);

    println!("Username: {}", user.username);
    println!("Age: {}", user.get_age());

    // calling enum.
    let current_state = LightState::On;
    print_state(current_state);

    let current_direction = Direction::West;
    print_direction(current_direction);

    //
    let cars = vec![
        Car {
            light_seen: TrafficLight::Red,
        },
        Car {
            light_seen: TrafficLight::Yellow,
        },
        Car {
            light_seen: TrafficLight::Green,
        },
    ];

    // for with the cars.
    for car in cars {
        car.react_to_light();
    }
}

// enum is used when we want to match every possible cases.
