use std::os::unix::thread;
/**
* // Reference counting Rc<String>
*
//  * Mutex: Mutex is still using thread fc, eg. when we implement the multiple thread but some of the threads are mutating the data then we have to use the Mutex -> mutual exclusion. Mutext is used with Locking.

* When to use Rc ?
-> when we don't want to mutate the data.
*/
use std::rc::Rc;
use std::time::{Duration, Instant};

struct Balloon {
    color: String,
}

fn main() {
    println!("Hello, world!");

    // Create a red ballon wrapped in Rc (Reference Counted pointer)
    let balloon = Rc::new("Red Balloon");

    println!("Original balloon: {}", balloon);
    println!("Reference count: {}\n", Rc::strong_count(&balloon));

    // Share the balloon with your sister.
    let sister_balloon = Rc::clone(&balloon);
    println!("Sister now has the balloon: {}", sister_balloon);
    println!("Reference count: {}", Rc::strong_count(&balloon));

    // Share the balloon with your brother.
    let brother_balloon = Rc::clone(&balloon);
    println!("Brother now has the balloon: {}", brother_balloon);
    println!("Reference count: {}\n", Rc::strong_count(&balloon));

    // Everyone can see the same balloon!
    println!("---- Everyone looks at their balloon ----");
    println!("You see: {}", balloon);
    println!("Sister sees: {}", sister_balloon);
    println!("Brother sees: {}", brother_balloon);

    // How many people are holding strings to the balloon?
    println!(
        "\nThe total number of references to the balloon: {}",
        Rc::strong_count(&balloon)
    );

    // When variables go out of scope, the reference count decrease.
    drop(sister_balloon);
    println!(
        "\nSister let go! references remaining: {}",
        Rc::strong_count(&balloon)
    );

    drop(brother_balloon);
    println!(
        "\nBrother let go! references remaining: {}",
        Rc::strong_count(&balloon)
    );

    println!("\n--- But nobody can pop (modify) it! ---");
    // Rc provides immutable shared access.
    // This would NOT works.
    // balloon.push("X"); // Error: cannot borrow as mutable.
    println!("Rc only allows reading, not modifying");
    println!("Everyone can look, but nobody can change it.");

    // The balloon will be automatically freed when the last references is dropped.

    println!("Birthday Party Balloon Story!\n");

    // Mom brings one balloon to the party.
    let party_balloon = Rc::new(Balloon {
        color: String::from("Blue"),
    });

    println!("At first, only Mom has the balloon!");
    println!("Balloon color: {}", party_balloon.color);
    println!(
        "People holding balloon: {}\n",
        Rc::strong_count(&party_balloon)
    );

    // Your friend wants to play with it.
    {
        let friend_balloon = Rc::clone(&party_balloon);
        println!("Your friend got a string to the balloon!");
        println!("Balloon color: {}", friend_balloon.color);
        println!(
            "People holding balloon: {}\n",
            Rc::strong_count(&party_balloon)
        );

        // You join in too!
        let my_balloon = Rc::clone(&party_balloon);
        println!("You got a string too!");
        println!("Balloon color: {}", my_balloon.color);
        println!(
            "People holding balloon: {}\n",
            Rc::strong_count(&party_balloon)
        );

        // Friend goes home (balloon string is dropped)
        println!("Friend went home and let go of their string.!");
    }

    println!("Single Thread - Must wait for each task\n");

    let start = Instant::now();

    download_file(); // Takes 5 seconds - WAITING...
    process_data(); // Takes 5 seconds - WAITING...
    save_to_disk(); // Takes 2 seconds - WAITING...

    let elapsed = start.elapsed();
    println!("\n total time: {:.1} seconds", elapsed.as_secs_f64());
    println!("(5+3+2=10seconds)\n");

    println!("====================================\n");
    println!("====================================\n");
    println!("====================================\n");

    multi_thread_example();
}

fn download_file() {
    println!("Downloading file...(5 seconds)");
    thread::sleep(Duration::from_secs(5));
    println!("Download complete!");
}

fn process_data() {
    println!("Processing file...(3 seconds)");
    thread::sleep(Duration::from_secs(3));
    println!("Processing complete!");
}

fn save_to_disk() {
    println!("Saving to disk.... (2 seconds)");
    thread::sleep(Duration::from_secs(2));
    println!("Saving to disk complete.!");
}

fn multi_thread_example() {
    println!("Multi-thread - All at same time!\n");

    let start = Instant::now();

    // spawn all threads at once.
    let handle1 = thread::spawn(|| {
        println!("Download file...(5 seconds");
        thread::sleep(Duration::from_secs(5));
        println!("Download complete..!");
    });

    let handle2 = thread::spawn(|| {
        println!("Processing data...(3 seconds");
        thread::sleep(Duration::from_secs(3));
        println!("Processing complete..!");
    });

    let handle3 = thread::spawn(|| {
        println!("Saving to disk...(2 seconds");
        thread::sleep(Duration::from_secs(2));
        println!("Saving complete..!");
    });

    // Wait for all threads to finish.
    handle1.join().unwrap();
    handle2.join().unrwap();
    handle3.join().unwrap();

    let elapsed = start.elapsed();
    println!("\n Total time: {:.1} seconds", elapsed.as_secs_f64());
    println!("(Only 5 seconds - the longest tasks !)\n");
    println!("2x faster with threads!");
}
