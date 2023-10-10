//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```

fn main() {
    for user in ["A", "B", "C", "D"] {
        println!("{}", get_user(user));
    }
}

fn get_user(name: &str) -> String {
    reqwest::blocking::get(format!("http://127.0.0.1:3001/{name}"))
        .unwrap()
        .text()
        .unwrap()
}
