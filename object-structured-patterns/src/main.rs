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

trait ShelbyWithDefault {
    fn speak(&self) {
        println!("I'm a Shelby by default so bear with me.");
    }
}

struct JohnShelby;

impl ShelbyWithDefault for JohnShelby {}

fn meet_and_greet(shelby: impl ShelbyWithDefault) {
    shelby.speak();
}

fn main() {
    let tommy = ThomasShelby;
    let arthur = ArthurShelby;
    tommy.speak();
    arthur.speak();

    let john = JohnShelby;
    john.speak();
    let jon = JohnShelby;
    meet_and_greet(jon);
}
