use std::thread::spawn;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };

    let handle = spawn(move || {
        println!("Hello from first thread: {:?}", user);
    });

    handle.join().unwrap();
    // println!("{:?}", user);

    let user_two = User {
        name: "Bob".to_string(),
        age: 25,
    };

    // without move so we can read
    let handle_two = spawn(|| {
        println!("Hello from second thread: {:?}", &user_two);
    });

    // without move so we can read
    let handle_three = spawn(|| {
        println!("Hello from third thread: {:?}", &user_two);
    });

    handle_two.join().unwrap();
    handle_three.join().unwrap();
}
