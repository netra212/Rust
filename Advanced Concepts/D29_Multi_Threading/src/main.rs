/**
 * // Reference counting Rc<String>
 *
//  * Mutex: Mutex is still using thread fc, eg. when we implement the multiple thread but some of the threads are mutating the data then we have to use the Mutex -> mutual exclusion. Mutext is used with Locking.

* When to use Rc ?
-> when we don't want to mutate the data.
 */
use std::rc::Rc;

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
}
