use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap(); // try_recv
    println!("Got: {received}");

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals: Vec<String> = vec!["hi", "from", "the", "thread"]
            .into_iter()
            .map(String::from)
            .collect();
        // tx.send(vals).unwrap();
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(move || {
        let vals: Vec<String> = vec!["more", "messages", "for", "you"]
            .into_iter()
            .map(String::from)
            .collect();
        // tx1.send(vals).unwrap();
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    for received in rx {
        // println!("{received:?}");
        println!("{received}");
    }
}
