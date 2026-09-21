use std::time::Instant; // to measure elapsed time.
use tokio::time::{Duration, sleep};
async fn make_drink(name: &'static str, seconds: u64) -> String {
    println!("{name}: started");

    sleep(Duration::from_secs(seconds)).await;

    println!("{name}: finished");

    format!("{name} is ready")
}

#[tokio::main]
async fn main() {
    println!("Creating latte Future...");

    let latte_future = make_drink("latte", 2);

    println!("Future created!");

    let latte = latte_future.await;
    println!("{latte}");

    let start = Instant::now();

    let tea_future = make_drink("Tea", 2);
    // /**
    // time ------------------------------------>

    // Tea
    // |--------- waiting 2 sec --------|
    //                                   DONE

    // Coffee
    // |-------------- waiting 3 sec ----------------|
    //                                                DONE
    // so, the total time is 3 second approximately.
    // Not the 2 + 3 = 5 seconds because when Tea is waiting, Coffee can also make progress.
    // */
    let coffee_future = make_drink("Coffee", 3);

    let (tea, coffee) = tokio::join!(tea_future, coffee_future); // Note: Future can contain other Futures. return of join here is like this: ("Tea is ready!", "Coffee is ready!")
    // Main function contains join!() here.
    // join! contains the Tea Future and Coffee Future and those contains Timer Futures.
    //

    println!("{tea}");
    println!("{coffee}");

    println!("Total: {:?}", start.elapsed());
}

// Poll -> It has two possibilities.
// Poll::Pending -> meaning I can't produce my value right now.
// Poll::Ready(value) -> meaning I am finished. Here is my final value.
// Conceptually:
// Executor:
// "Socket, are you ready ?"
// Socket:
// "No"
// "Here's my registration with the OS."
// "When data arrives, wake this task"
// Executor:
// "Okay"
// ....later....
// OS:
// "Socket readable"
// Runtime:
// waker.wake()
// Scheduler:
// put Task A onto runable queue
// Executor:
// poll Task A again

/*
# Executor + Future + Waker together
                 ┌───────────────┐
                 │   Executor    │
                 │ / Scheduler   │
                 └───────┬───────┘
                         │
                         │ poll()
                         ▼
                 ┌───────────────┐
                 │    Future     │
                 └───────┬───────┘
                         │
                ┌────────┴────────┐
                │                 │
                ▼                 ▼
        Poll::Ready(x)      Poll::Pending
                                  │
                                  │
                                  ▼
                         register Waker
                                  │
                                  │
                        ...time passes...
                                  │
                         socket/timer/etc
                                  │
                                  ▼
                             wake()
                                  │
                                  ▼
                            Executor
                                  │
                                  ▼
                             poll again
*/
