enum state {
    connected,
    disconnected,
    connecting,
}

fn main() {
    println!("\n------ Enums in Rust ------");

    let system_connection = state::connecting;

    match system_connection {
        state::connected => println!("\nYour system is connected."),
        state::disconnected => println!("\nYour system is disconnceted."),
        state::connecting => println!("\nYour system is connecting."),
    }
}
