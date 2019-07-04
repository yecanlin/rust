use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        //thread::sleep(Duration::from_secs(5));
        tx.send(val).unwrap();
        //println!("use the val again: {}", val);
    });

    println!("run here");
    let received = rx.recv().unwrap();
    
    println!("Got: {}", received);
}

