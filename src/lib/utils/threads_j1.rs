// main.rs
use rand::{self, Rng};
use std::{thread, thread::JoinHandle, time::Duration};

/* create function that not return anything */
pub fn foo_threads_collect() {
    // do something
    let handles: Vec<JoinHandle<String>> = (0..=15)
        .map(|i| {
            let delay = rand::thread_rng().gen_range(10..=2000);
            let builder = thread::Builder::new().name(format!("Thread-{}", i));

            builder
                .spawn(move || {
                    println!("thread started = {}", thread::current().name().unwrap());
                    thread::sleep(Duration::from_millis(delay));
                    thread::current().name().unwrap().to_owned()
                })
                .unwrap()
        })
        .collect();
    for h in handles {
        let r = h.join().unwrap();
        println!("thread done = {:?}", r);
    }
}

pub fn foo_threads_unwrap() {
    let handle: JoinHandle<i32> = thread::spawn(|| {
        let delay = rand::thread_rng().gen_range(10..=2000);
        thread::sleep(Duration::from_millis(delay));
        println!("Hello from spawned thread");
        5
    });
    println!("return = {}", handle.join().unwrap());
}

pub fn foo_threads_fork_join() {
    let first_name_handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        "Kevin"
    });
    let last_name_handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        "Greene"
    });
    let name = format!(
        "{} {}",
        first_name_handle.join().unwrap(),
        last_name_handle.join().unwrap()
    );
    println!("name = {}", name);
}
