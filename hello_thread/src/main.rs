use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from thread! {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("Hello, world!");

    for i in 1..5 {
        println!("Hello from main thread! {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
