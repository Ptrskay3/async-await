// Run with: `cargo +nightly -Zscript manual_scoped.rs`

//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```

fn main() {
    std::thread::scope(|scope| {
        for name in &["A", "B", "C", "D"] {
            scope.spawn(move || println!("{}", fry_egg(name)));
        }
    });
}

fn fry_egg(name: &str) -> String {
    reqwest::blocking::get(format!("http://127.0.0.1:3001/{name}"))
        .unwrap()
        .text()
        .unwrap()
}
