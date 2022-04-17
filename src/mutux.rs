use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        threads.push(t);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("Result {}", *c.lock().unwrap());
}