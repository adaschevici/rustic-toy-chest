use crossbeam::scope;
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

    // This will not compile because we are trying to move user_two
    // that is not allowed, but if we don't move it we get argument
    // required to outlive 'static
    // let user_two = User {
    //     name: "Bob".to_string(),
    //     age: 25,
    // };
    //
    // // without move so we can read
    // let handle_two = spawn(|| {
    //     println!("Hello from second thread: {:?}", &user_two);
    // });
    //
    // // without move so we can read
    // let handle_three = spawn(|| {
    //     println!("Hello from third thread: {:?}", &user_two);
    // });
    //
    // handle_two.join().unwrap();
    // handle_three.join().unwrap();

    let user_three = User {
        name: "Charlie".to_string(),
        age: 20,
    };

    scope(|s| {
        s.spawn(|_| {
            println!("Hello from fourth thread: {:?}", &user_three);
        });

        s.spawn(|_| {
            println!("Hello from fifth thread: {:?}", &user_three);
        });
    })
    .unwrap();
}
