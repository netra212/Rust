// #[derive(Default)]
// struct Event {
//     name: String,
//     date: String,
//     location: String,
// }

// #[derive(Debug, Default)]
// struct Character {
//     name: String,
//     health: u32,
//     power: u32,
// }

// struct Book {
//     pub title: String,
//     pub author: String,
//     pub author: i32,
// }

// impl Book {
//     pub fn new(title: &str, author: &str, year: i32) -> Self {
//         Book {
//             title: title.into(),
//             author: author.into(),
//             year,
//         }
//     }

//     pub fn open(&self) {
//         println!(
//             "The Book {} was written by {} in {}.",
//             self.title, self.author, self.year
//         );
//     }
// }

// struct Color(u8, u8, u8);

// struct User {
//     pub username: String,
//     age: u8,
// }

// impl User {
//     pub fn new(username: String, age: u8) -> Self {
//         User { username, age }
//     }
// }

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

fn main() {
    //
    // let event = Event {
    //     name: String::from("Rust Meetup."),
    //     date: String::from("11-05-2026"),
    //     location: String::from("London"),
    // };

    // println!("Event Location: {}", event.name);
    // println!("Event Location: {}", event.date);
    // println!("Event Location: {}", event.location);

    // let default_character = Character::default();
    // println!("Default: {default_character:?}");

    // let hero = Character {
    //     name: String::from("SuperHero"),
    //     ..Default::default()
    // };

    // println!("Hero: {hero:?}");

    // let ancient_book = Book::new("Title Ancient", "Ancient Book", 234);
    // let modern_book = Book::new("Title Modern", "Modern Book", 2024);

    // ancient_book.open();
    // modern_book.open();

    // let sky_blue = Color(135, 235, 60);
    // println!("Red: {}", sky_blue.0);
    // println!("Green: {}", sky_blue.1);
    // println!("Blue: {}", sky_blue.2);

    let user = user_module::User::new(String::from("Alice"), 34);

    println!("Username: {}", user.username);
    println!("Age: {}", user.get_age());
}
