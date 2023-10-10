//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
//! ```
use tokio::time::{Duration};
use std::sync::Mutex;

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(0);

    tokio::join!(work(&mutex), work(&mutex));

    println!("{}", *mutex.lock().unwrap());
}

async fn work(mutex: &Mutex<i32>) {
    let mut v = mutex.lock().unwrap();
    println!("locked");
    // slow network request
    tokio::time::sleep(Duration::from_millis(100)).await;
    *v += 1;
}
