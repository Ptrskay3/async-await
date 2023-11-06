//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
//! reqwest = { version = "0.11" }
//! futures = { version = "0.3" }
//! ```

use futures::future::join_all;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let tasks = ["A", "B", "C", "D"].into_iter().map(|name| async {
        fry_egg(name).await
    });
    let users = join_all(tasks).await;
    println!("{users:?}");
}

async fn fry_egg(name: &str) -> String {
    reqwest::get(format!("http://127.0.0.1:3001/{name}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
