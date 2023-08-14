//use models::Person;

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub salary: u32,
}
