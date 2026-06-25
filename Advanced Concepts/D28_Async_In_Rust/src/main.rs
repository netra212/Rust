use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{Duration, sleep};

/**
 * Async Rust.
 * Understanding poll()
 * Understanding the Waker()
 * Pinning - This is a mechanism for attaching data to a specific locationo in memory. By using Pin, you gurantee that the object won't be moved to another location in memory. For example, Pin<&mut Self> indicates that &mut Self will always be in the same memory location between the calls to Self::poll. Using the same memory location is mandatory if you want to be sure that the fields of Self don't change their memory address.
 *
 */

async fn long_running_task(id: i32) {
    println!("Task {} started", id);

    for i in 1..=10 {
        sleep(Duration::from_millis(500)).await;
        println!("Task {} - step {}", id, i);
    }

    println!("Task {} completed", id);
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

fn regular_function() -> i32 {
    println!("Regular function executing NOW");
    42
}

async fn async_function() -> i32 {
    println!("Regular function executing NOW");
    42
}

// BLOCKING - stops everything, wait doing nothing.
fn blocking_task(id: i32) {
    println!("Blocking task {} started", id);
    std::thread::sleep(Duration::from_secs(2)); // Thread is frozen for 2 seconds. 
    println!("Blocking task {} finished", id);
}

// NON-BLOCKING - stops everything, wait doing nothing.
async fn non_blocking_task(id: i32) {
    println!("Non-Blocking task {} started", id);
    tokio::time::sleep(Duration::from_secs(2)).await; // Thread is frozen for 2 seconds. 
    println!("Non-Blocking task {} finished", id);
}

// Custom Future that counts to 3.
struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.count += 1;
        println!("Polling... count is now {}", self.count);

        if self.count < 3 {
            // Not ready yet - wake ourselves to be polled again.
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            // Ready
            Poll::Ready(self.count)
        }
    }
}
#[tokio::main]
async fn main() {
    println!("=== Regular Function ===");
    let result = regular_function();
    println!("Result: {}\n", result);

    println!("=== Async Function (Future) ===");
    let future = async_function();
    println!("Future created, but async_function hasn't run yet!");

    println!("Now calling .await...");
    let result = future.await; // Now it executes. 
    println!("Result: {}", result);

    println!("=== BLOCKING CODE ===");
    let start = std::time::Instant::now();

    blocking_task(1); // wait for 2 seconds. 
    blocking_task(2); // wait for 2 seconds. 
    blocking_task(3); // wait for 2 seconds. 

    println!("Total time: {:?}\n", start.elapsed());
    // Each task waits, one after another = 6 seconds total.

    println!("=== NON-BLOCKING CODE ===");
    let start = std::time::Instant::now();

    // Run all tasks concurrently.
    tokio::join!(
        non_blocking_task(1),
        non_blocking_task(2),
        non_blocking_task(3),
    );

    println!("Total time: {:?}", start.elapsed());
    // All tasks run at the same time = only 2 seconds total!

    //
    println!("=== Without Cancellation ===");
    long_running_task(1).await;
    println!();

    println!("=== With Cancellation ===");

    // create a task handle.
    let task = tokio::spawn(long_running_task(2));

    // Let ir run for a bit.
    println!("Cancelling tasks 2...");
    task.abort();

    // Check if cancelled.
    match task.await {
        Ok(_) => println!("Task completed normally"),
        Err(e) if e.is_cancelled() => println!("Task was cancelled"),
        Err(e) => println!("Task failed: {}", e),
    }

    let future1 = CounterFuture { count: 0 };
    let res1 = future1.await;
    println!("Final Result: {}", result);
}
