// Run with: `cargo +nightly -Zscript manual_chan.rs`

//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```

use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();

    std::thread::scope(|scope| {
        for name in &["A", "B", "C", "D"] {
            let sender = sender.clone();
            scope.spawn(move || sender.send(fry_egg(name)).unwrap());
        }
    });

    // Close the channel, indicating we don't expect more messages.
    drop(sender);
    while let Ok(item) = receiver.recv() {
        println!("{item}");
    }
}

fn fry_egg(name: &str) -> String {
    reqwest::blocking::get(format!("http://127.0.0.1:3001/{name}"))
        .unwrap()
        .text()
        .unwrap()
}
