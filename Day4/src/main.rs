enum state {
    connected,
    disconnected,
    connecting,
}

enum Traffic {
    Red,
    Green,
    Yellow,
}

// enum -> can hold value.
enum Message {
    Quit, // No data
    Text(String, i32, i32), // Tuples
    Move{x: i32, y:i32}, // Structure
}

fn matching(msg: Message) {
    match msg {
        Message::Quit => prinln!("There is no message"),
        // - => {}, // I do not want to perform any more operations. 
        Message::Text(content, x, y) => prinln!("Our {} {} {}", content, x, y), 
        Message::Move{x, y} => prinln!("Value are:  {} {}", x, y), 
    }
}

enum Option<T> {
    Some(T), // T can be anything either String or tuples, T -> template. 
    None,
    Age{x:i32, y:i32}
}

fn matching_option<T>(msg: &Option<T>){
    match msg{
        Option::Some(value) => println!("{}", value),
        Option::None => println!("Nothing to display"),
        Option::Age(x, y) => println!("{} {}", x, y),
    }
}

fn main() {
    println!("\n------ Enums in Rust ------");

    let system_connection = state::connecting;

    match system_connection {
        state::connected => println!("\nYour system is connected."),
        state::disconnected => println!("\nYour system is disconnceted."),
        state::connecting => println!("\nYour system is connecting."),
    }

    // Traffic Light System.
    let traffic_light = Traffic::Red;

    let update = match traffic_light {
        Traffic::Red => String::from("\nSTOP"),
        Traffic::Green => String::from("\nGO"),
        Traffic::Yellow => String::from("\nSLOW"),
    };

    println!("{}", update);

    let msg1 = Message::Text(String::from("Netra Khatri"), 30, 70);
    let msg2 = Message:Quit;
    let msg3 = Message::Move(x:20, y:40);

    matching(&msg1);
    matching(&msg2);
    matching(&msg3);

    // Template with Options. 
    let m1 = Option::Some(String::from("Hello Ji"));
    let m2: Option<i32> = Number::Age{x:20, y:40};
    let m3: Option<i32> = Option::None;

    matching_option(&m1);
    matching_option(&m2);
    matching_option(&m3);
}
