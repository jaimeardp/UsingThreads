//use serde::Serialize;
use std::fs::File;
use std::io::Write;


use crate::lib::models;

//use crate::models::Person;

pub fn write_json_to_file(persons: &Vec<models::Person>) {
    // Create an instance of the struct
    /*let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };*/

    // Convert the struct to a JSON string
    let json_data = serde_json::to_string_pretty(&persons).unwrap();

    // Print the JSON string
    println!("{}", json_data);

    save_to_file("person.json", &json_data);
}

fn save_to_file(filename: &str, data: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(data.as_bytes())
        .expect("Unable to write data");
}
