use workspace_macros::ToJson;

pub trait ToJson {
    fn to_json(&self) -> String;
}

#[derive(ToJson)]
struct Person {
    name: String,
    age: u8,
    is_alive: bool,
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
