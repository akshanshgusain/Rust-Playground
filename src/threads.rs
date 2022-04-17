use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let mut threads = vec![];
    for i in 0..10 {
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(i * 1000));
            println!("new thread {}", i);
        });
        threads.push(th);
    }

    for t in threads {
        t.join();
    }

    println!("Main thread");
}