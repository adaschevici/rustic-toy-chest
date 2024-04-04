trait Shelby {
    fn speak(&self);
}

struct ThomasShelby;

struct ArthurShelby;

impl Shelby for ThomasShelby {
    fn speak(&self) {
        // What Tommy says
        println!("I'm Thomas Shelby, and this is my business.");
    }
}

impl Shelby for ArthurShelby {
    fn speak(&self) {
        // What Arthur says
        println!("I'm Arthur Shelby, and I fight for my family.");
    }
}

fn main() {
    let tommy = ThomasShelby;
    let arthur = ArthurShelby;
    tommy.speak();
    arthur.speak();
}
