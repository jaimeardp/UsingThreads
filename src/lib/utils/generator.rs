// main.rs
extern crate uuid;

use uuid::Uuid;

use crate::lib::models;
use rand::{self, Rng};
use std::{thread, thread::JoinHandle, time::Duration};

fn generate_random_number(min: u8, max: u8) -> u8 {
    let rand_num = rand::thread_rng().gen_range(min..max);
    rand_num
    //println!("Random number: {}", rand_num);
}

fn gen_data() -> Vec<models::Person> {
    let mut persons_vec = vec![];
    //let id_person = generate_random_number(1u8, 99u8);
    for i in 0..150 {
        let id_person = generate_random_number(1u8, 99u8) + i;
        let person = models::Person {
            id: Uuid::new_v4().to_string(),
            name: format!("first_name_{}, last_name_{}", id_person, id_person),
            age: generate_random_number(1u8, 99u8),
            salary: generate_random_number(0u8, 255u8) as u32,
        };
        //println!("{:?}", persons);
        persons_vec.push(person);
    }
    persons_vec
}

/* create function that not return anything */
pub fn threads_data_collect(iterationes: u32) -> Vec<models::Person> {
    // do something
    let mut result_data: Vec<models::Person> = Vec::new();

    let handles: Vec<JoinHandle<Vec<models::Person>>> = (0..=iterationes)
        .map(|i| {
            let delay = rand::thread_rng().gen_range(10..=2000);
            let builder = thread::Builder::new().name(format!("Thread-{}", i));

            builder
                .spawn(move || {
                    println!("thread started = {}", thread::current().name().unwrap());
                    //thread::sleep(Duration::from_millis(delay));
                    //thread::current().name().unwrap().to_owned();
                    gen_data()
                })
                .unwrap()
        })
        .collect();
    for h in handles {
        let mut r = h.join().unwrap();
        // flatten r
        result_data.append(&mut r);
        //result_data.push(r);
        //println!("thread done = {:?}", r);
    }
    result_data
}
