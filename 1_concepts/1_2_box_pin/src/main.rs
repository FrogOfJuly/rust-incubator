use std::{fmt, future::Future, pin::Pin, rc::Rc};

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

// impl<T> SayHi for T
// where
//     T: fmt::Debug,
// {
//     fn say_hi(self: Pin<&Self>) {
//         println!("Hi from {:?}", self)
//     }
// }

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Implementation must be meaningful, and
        // obviously call something requiring `&mut self`.
        // The point here is to practice dealing with
        // `Pin<&mut Self>` -> `&mut self` conversion
        // in different contexts, without introducing
        // any `Unpin` trait bounds.
    }
}

impl<T: fmt::Debug> SayHi for Box<T> {}

impl<T: Default> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Box<T>>) {
        let x = self.get_mut();
        *x = Box::default();
    }
}

impl<T: fmt::Debug> SayHi for Rc<T> {}

impl<T: Default> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Rc<T>>) {
        let x = self.get_mut();
        *x = Self::default();
    }
}

impl<T: fmt::Debug> SayHi for Vec<T> {}

impl<T> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(self: Pin<&mut Vec<T>>) {
        // let x = self.get_mut();
        // x.clear();
        //Vector implements unpin only for types that implement unpin
    }
}

impl SayHi for String {}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut String>) {
        let x = self.get_mut();
        x.clear();
    }
}

impl SayHi for &[u8] {}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let x = self.get_mut();
        *x = &[1, 2, 3, 4];
    }
}

#[pin_project::pin_project]
struct MeasurableFuture<Fut> {
    #[pin]
    inner_future: Fut,
    started_at: Option<std::time::Instant>,
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();

        let start = this.started_at.get_or_insert_with(std::time::Instant::now);
        let elapsed = start.elapsed().as_secs_f32();

        use std::task::Poll::Pending;
        use std::task::Poll::Ready;
        match this.inner_future.poll(cx) {
            Ready(v) => {
                println!("{:.5}", elapsed);
                Ready(v)
            }
            Pending => Pending,
        }
    }
}

fn main() {
    println!("Implement me!");
}
