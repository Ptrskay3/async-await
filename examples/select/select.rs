//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
//! reqwest = { version = "0.11" }
//! ```

use tokio::select;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let result = select! {
        r = get_user("A") => r,
        r = get_user("B") => r,
        r = get_user("C") => r,
        r = get_user("D") => r
    };

    println!("{result}");
}

async fn get_user(name: &str) -> String {
    reqwest::get(format!("http://127.0.0.1:3001/{name}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
