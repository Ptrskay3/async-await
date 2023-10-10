//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```

use std::sync::mpsc::channel;

fn main() {
    let mut handles = vec![];
    let (sender, receiver) = channel();

    for name in &["A", "B", "C", "D"] {
        let sender = sender.clone();
        let handle = std::thread::spawn(move || sender.send(get_user(name)).unwrap());
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    drop(sender);

    while let Ok(item) = receiver.recv() {
        println!("{item}");
    }
}

fn get_user(name: &str) -> String {
    reqwest::blocking::get(format!("http://127.0.0.1:3001/{name}"))
        .unwrap()
        .text()
        .unwrap()
}
