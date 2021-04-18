use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU32, Ordering},
    task::{Context, Poll},
};
use futures::pin_mut;

use cassette::CasMachine;

struct Demo {
    lol: u32,
}

impl Demo {
    async fn entry(&mut self) {
        for _ in 0..10 {
            self.entry_1().await;
            self.entry_2().await;
        }
    }

    async fn entry_1(&mut self) {
        self.start_at_zero().await;
        self.add_one_until_ten().await;
        self.sub_one_until_zero().await;
    }

    async fn entry_2(&mut self) {
        self.start_at_five().await;
        self.sub_one_until_zero().await;
        self.add_one_until_ten().await;
    }

    async fn start_at_zero(&mut self) {
        self.lol = 0;
    }

    async fn start_at_five(&mut self) {
        self.lol = 5;
    }

    async fn add_one_until_ten(&mut self) {
        loop {
            cats(self).await; // simulate fake delays/not ready state
            self.lol += 1;
            if self.lol >= 10 {
                return;
            }
        }
    }

    async fn sub_one_until_zero(&mut self) {
        loop {
            cats(self).await; // simulate fake delays/not ready state
            self.lol -= 1;
            if self.lol == 0 {
                return;
            }
        }
    }
}

fn main() {
    let mut demo = Demo { lol: 100 };
    let x = demo.entry();
    pin_mut!(x);

    let mut cm = CasMachine::new(x);

    loop {
        if let Some(x) = cm.poll_on() {
            println!("Done!: `{:?}`", x);
            break;
        }
    }
}

static FAKE: AtomicU32 = AtomicU32::new(0);

async fn cats(ctxt: &mut Demo) {
    println!("cats says lol: {}", ctxt.lol);
    let x = CountFuture;
    x.await;
    println!("and cats!");
}

struct CountFuture;

impl Future for CountFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let x = FAKE.fetch_add(1, Ordering::SeqCst);
        print!("{}, ", x);
        if (x % 5) == 0 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}