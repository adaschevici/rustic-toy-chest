struct Originator {
    state: String,
}

// This is the object that will be saved and restored
impl Originator {
    fn new() -> Originator {
        Originator {
            state: String::new(),
        }
    }

    fn set_state(&mut self, state: String) {
        self.state = state;
    }

    fn save_to_memento(&self) -> Memento {
        Memento {
            state: self.state.clone(),
        }
    }

    fn restore_from_memento(&mut self, memento: Memento) {
        self.state = memento.state;
    }
}

// This is the object that will wrap the state of the Originator
struct Memento {
    state: String,
}

impl Memento {
    fn new(state: String) -> Memento {
        Memento { state }
    }

    fn get_state(&self) -> String {
        self.state.clone()
    }
}

fn main() {
    println!("Hello, world!");
}
