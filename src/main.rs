// main.rs
//use rand::{self, Rng};
//use std::{thread, thread::JoinHandle, time::Duration};

// import function of other file rs
// Declare the module
//mod threads_j1;
//mod clousure_j1;
pub mod lib;
// Import specific functions from the module
//use threads_j1::{foo_threads_collect, foo_threads_fork_join, foo_threads_unwrap};
//use clousure_j1::{clousure_force_ownership, clousure_mutable, clousure_print};

//use models::Person;

use crate::lib::models;
use crate::lib::utils;

fn main() {
    /*let persons = vec![
        models::Person {
            name: String::from("Hello, "),
            age: 1,
        },
        models::Person {
            name: String::from("Hello, "),
            age: 5,
        },
        models::Person {
            name: String::from("Hello, "),
            age: 12,
        },
    ];*/
    //foo_threads_collect();
    //foo_threads_unwrap();
    //foo_threads_fork_join();
    //clousure_print();
    //clousure_mutable();
    //clousure_force_ownership();

    let persons_response = utils::threads_data_collect(100);

    utils::write_json_to_file(&persons_response);
}
