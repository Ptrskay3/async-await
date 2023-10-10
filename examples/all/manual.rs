//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```

fn main() {
    let mut handles = vec![];

    for name in &["A", "B", "C", "D"] {
        let handle = std::thread::spawn(move || get_user(name));
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("{result}");
    }
}

fn get_user(name: &str) -> String {
    reqwest::blocking::get(format!("http://127.0.0.1:3001/{name}"))
        .unwrap()
        .text()
        .unwrap()
}
