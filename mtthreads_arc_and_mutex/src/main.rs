#![feature(negative_impls)]
use crossbeam::scope;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep, spawn};
use std::time::Duration;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct UserTwo {
    name: RefCell<String>,
    age: Cell<u32>,
}

#[derive(Debug)]
struct Foo {}

impl !Send for Foo {}

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

    let user_four_original = Arc::new(User {
        name: "David".to_string(),
        age: 15,
    });

    let user_four = user_four_original.clone();

    let handle_four = spawn(move || {
        println!("Hello from sixth thread: {:?}", &user_four);
    });

    let user_four = user_four_original.clone();
    let handle_five = spawn(move || {
        println!("Hello from seventh thread: {:?}", &user_four);
    });

    handle_four.join().unwrap();
    handle_five.join().unwrap();

    // this is not allowed to be shared across threads
    // let foo_one = Foo {};
    // let handle_six = spawn(move || {
    //     dbg!(foo_one);
    // });
    // handle_six.join().unwrap();
    //
    // this is still not allowed to be shared across threads
    // let foo_two = Arc::new(Foo {});
    // let handle_seven = spawn(move || {
    //     dbg!(foo_two);
    // });
    let user_five_original = Arc::new(Mutex::new(User {
        name: "Eve".to_string(),
        age: 10,
    }));
    let user_five = user_five_original.clone();
    let handle_eight = spawn(move || {
        let mut locked_user_five = user_five.lock().unwrap();
        locked_user_five.name = "Evee".to_string();
        // user five cannot be locked multiple times because it will hang the program
        // println!("Hello from eighth thread: {:?}", &user_five);
    });
    let user_five = user_five_original.clone();
    let handle_nine = spawn(move || {
        sleep(Duration::from_secs(1));
        // user five cannot be locked multiple times because it will hang the program
        // let mut locked_user_five = user_five.lock().unwrap();
        // locked_user_five.name = "Eveee".to_string();
        println!("Hello from ninth thread: {:?}", &user_five.lock().unwrap());
    });
    handle_eight.join().unwrap();
    handle_nine.join().unwrap();

    let user_six = Mutex::new(User {
        name: "Frank".to_string(),
        age: 5,
    });

    scope(|s| {
        s.spawn(|_| {
            user_six.lock().unwrap().name = "Frankk".to_string();
        });

        s.spawn(|_| {
            sleep(Duration::from_secs(1));
            println!("Hello from tenth thread: {:?}", &user_six.lock().unwrap());
        });
    })
    .unwrap();

    // let user_seven_original = Arc::new(UserTwo {
    //     name: RefCell::new("Grace".to_string()),
    //     age: Cell::new(50),
    // });
    //
    // let user_seven = user_seven_original.clone();
    // thread::spawn(move || {
    //     user_seven.name.replace("Gracee".to_string());
    // });
    // let user_seven = user_seven_original.clone();
    // thread::spawn(move || {
    //     user_seven.age.set(55);
    // });
}

