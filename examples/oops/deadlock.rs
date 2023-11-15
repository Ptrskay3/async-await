//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
//! ```
// To run through Miri: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a3f282dfad6bfc2f96c8ca871de06569

use tokio::{join, time::{Duration}};
use std::sync::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mutex = Mutex::new(0);

    join!(work(&mutex), work(&mutex));

    println!("{}", *mutex.lock().unwrap());
}

async fn work(mutex: &Mutex<i32>) {
    let mut v = mutex.lock().unwrap();
    println!("locked");
    tokio::time::sleep(Duration::from_millis(100)).await;
    *v += 1;
    // The mutex is unlocked here, at the end of scope
}
