use serde_json;
use workspace_macros::{ToJson, ToJsonGeneric};

pub trait ToJson {
    fn to_json(&self) -> String;
}
pub trait ToJsonGeneric {
    fn to_json(&self) -> String;
}

#[derive(ToJson)]
struct Person {
    name: String,
    age: u8,
    is_alive: bool,
}

#[derive(ToJsonGeneric)]
struct ZombiePerson {
    name: String,
    age: u8,
    is_alive: bool,
    is_zombie: bool,
}

pub async fn run_jsonify_macro() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_alive: true,
    };
    let json = person.to_json();
    println!("{}", json);
}

pub async fn run_jsonify_generic_macro() {
    let zombie_person = ZombiePerson {
        name: "Bob".to_string(),
        age: 30,
        is_alive: false,
        is_zombie: true,
    };
    let json = zombie_person.to_json();
    println!("{}", json);
}
