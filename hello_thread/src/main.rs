use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let handler2 = thread::spawn(move || {
        println!("Thread received vector: {:?}", v);
    });

    let handler = thread::spawn(|| {
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

    handler.join().unwrap();
    println!("Thread finished execution.");
}
