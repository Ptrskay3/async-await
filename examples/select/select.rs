// Run with: `cargo +nightly -Zscript select.rs`
//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
//! reqwest = { version = "0.11" }
//! ```

use tokio::select;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let result = select! {
        r = fry_egg("A") => r,
        r = fry_egg("B") => r,
        r = fry_egg("C") => r,
        r = fry_egg("D") => r
    };

    println!("{result}");
}

async fn fry_egg(name: &str) -> String {
    reqwest::get(format!("http://127.0.0.1:3001/{name}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
