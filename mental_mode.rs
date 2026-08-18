/*
# 1. The overall Rust mental model

                         RUST TYPES
                             │
              ┌──────────────┼──────────────┐
              │              │              │
            enum           struct         primitives
              │
              │
       ┌──────┴──────┐
       │             │
     Option        Result
       │
       │
   Some(T) / None
       │
       ▼
 PATTERN MATCHING
       │
       ▼
    match
       │
       ▼
  "Which variant
   do I have?"

// Ownership.

OWNERSHIP
    │
    ├── move
    │
    ├── borrow
    │      │
    │      ├── &T
    │      └── &mut T
    │
    └── LIFETIME
           │
           └── "How long is this reference valid?"


# 2. ENUMS
enum Token {
    LeftParen,
    Number(i64),
    Identifier(String),
}

let token = Token::Number(42);

Token
 │
 └── Number
       │
       └── 42

match token {
    Token::LeftParen => println!("("),

    Token::Number(value) => {
        println!("number = {}", value);
    }

    Token::Identifier(name) => {
        println!("identifier = {}", name);
    }
}

Think of match like a decision tree:

                       token
                         │
                         ▼
                 ┌───────────────┐
                 │ What variant? │
                 └───────┬───────┘
                         │
          ┌──────────────┼──────────────┐
          │              │              │
          ▼              ▼              ▼
    LeftParen        Number(value)   Identifier(name)
          │              │              │
          ▼              ▼              ▼
       print "("      use value      use name


let token = Token::Number(42);
match token {
    Token::Number(value) => {
        println!("{}", value);
    }
    _ => {}
}

Rust sees:
Token::Number(42)
        │
        │ pattern
        ▼
Token::Number(value)
                │
                ▼
             value = 42

# OPTION — one of two possibilities.
enum Option<T> {
    Some(T),
    None,
}

Option<i32> means
Option<i32>
     │
     ├──────────────┐
     │              │
     ▼              ▼
 Some(i32)         None
     │
     ▼
    42

                  Option<String>
                       │
              ┌────────┴────────┐
              │                 │
              ▼                 ▼
          Some(String)         None
              │
              ▼
            "Tony"

The compiler knows:
    "You must account for the possibility that the value doesn't exist."

# 9. Option + pattern matching.
These two concepts are naturally connected.
let name = Some("Tony");

match name {
    Some(value) => println!("Name: {}", value),
    None => println!("No name"),
}

                      name
                       │
                       ▼
                ┌────────────┐
                │   match    │
                └─────┬──────┘
                      │
             ┌────────┴────────┐
             │                 │
             ▼                 ▼
          Some(value)         None
             │                 │
             ▼                 ▼
        value = "Tony"     no value
             │                 │
             ▼                 ▼
          println!          println!

# 10. impl — attach behavior to your type
struct User {
    name: String,
    age: u32,
}

Without impl, you have the data:

User
 │
 ├── name
 └── age

With:

impl User {
    fn greet(&self) {
        println!("Hello {}", self.name);
    }
}

                    User
                     │
          ┌──────────┴──────────┐
          │                     │
        DATA                  BEHAVIOR
          │                     │
      ┌───┴───┐              ┌──┴────┐
      │       │              │       │
     name    age           greet()   ...

So, "impl" basically says like this
    "Here are the operations that belong to this type."
*/
