use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct ThreePollFuture {
    polls: u8,
}

impl Future for ThreePollFuture {
    type Output = &'static str;

    fn Poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls += 1;
        println!("poll called: {}", self.polls);

        if self.polls < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready("Future Finished")
        }
    }
}

#[tokio::main]
async fn main() {
    let future = ThreePollFuture { polls: 0 };

    let result = future.await;
    println!("{result}");
}
