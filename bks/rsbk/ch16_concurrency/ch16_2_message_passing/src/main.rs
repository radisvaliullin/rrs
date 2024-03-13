use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    // v1
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // commented, will panic because we send ownership val to tx.send
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("got: {}", received);

    // v2
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
        println!("got: {}", received);
    }

    // v3
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("thread0: 1"),
            String::from("thread0: 2"),
            String::from("thread0: 3"),
            String::from("thread0: 4"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("thread1: 1"),
            String::from("thread1: 2"),
            String::from("thread1: 3"),
            String::from("thread1: 4"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("got: {}", received);
    }

}
