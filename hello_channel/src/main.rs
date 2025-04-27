use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();

        let vals = vec!["hello", "world", "from", "Rust"];
        for val in vals {
            tx.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}
