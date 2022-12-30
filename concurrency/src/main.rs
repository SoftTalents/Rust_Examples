use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    let v = vec![1,2,3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap(); // block the currently running thread until the handled thread terminates.
    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {received}");
    }
    
    // let received = rx.recv().unwrap(); // block the main thread while waiting for a message
    println!("Hello, world!");
}
