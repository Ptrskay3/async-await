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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let stream = futures::stream::iter(["A", "B", "C", "D"]).then(get_user);
    // let stream = futures::stream::iter(["A", "B", "C", "D"]).map(get_user).buffered(2);
    // let stream = futures::stream::iter(["A", "B", "C", "D"]).map(get_user).buffer_unordered(2);
    // let stream = futures::stream::iter(["A", "B", "C", "D"]).map(get_user).buffered(2).chunks(2);
    pin_mut!(stream);
    
    while let Some(user) = stream.next().await {
        println!("{user:?}");
    }

    println!("{:?}", stream.collect::<Vec<_>>().await);
}

async fn get_user(name: &str) -> String {
    reqwest::get(format!("http://127.0.0.1:3001/{name}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
