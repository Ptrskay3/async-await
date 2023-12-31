// Run with: `cargo +nightly -Zscript stream.rs`
//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
//! reqwest = { version = "0.11" }
//! futures = { version = "0.3" }
//! futures-util = { version = "0.3" }
//! ```

use futures::stream::StreamExt;
use futures_util::pin_mut;

#[tokio::main]
async fn main() {
    let stream = futures::stream::iter(['A', 'B', 'C', 'D']).then(fry_egg);
    // let stream = futures::stream::iter(['A', 'B', 'C', 'D']).map(fry_egg).buffered(2);
    // let stream = futures::stream::iter(['A', 'B', 'C', 'D']).map(fry_egg).buffer_unordered(2);
    // let stream = futures::stream::iter(['A', 'B', 'C', 'D']).map(fry_egg).buffered(2).chunks(2);
    // let stream = futures::stream::iter('A'..'Z').filter(|&egg| async move { ('E'..='S').contains(&egg) }).map(fry_egg).buffered(5);
    pin_mut!(stream);
    
    while let Some(egg) = stream.next().await {
        println!("{egg:?}");
    }

    // println!("{:?}", stream.collect::<Vec<_>>().await);
}

async fn fry_egg(name: char) -> String {
    reqwest::get(format!("http://127.0.0.1:3001/{name}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
