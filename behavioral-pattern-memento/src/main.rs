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

// This is the object that will hold the Memento objects as a collection
// and will be responsible for saving and restoring the Originator state
struct Caretaker {
    mementos: Vec<Memento>,
}

impl Caretaker {
    fn new() -> Caretaker {
        Caretaker {
            mementos: Vec::new(),
        }
    }

    fn add_memento(&mut self, memento: Memento) {
        self.mementos.push(memento);
    }

    fn get_memento(&self, index: usize) -> Memento {
        self.mementos[index].clone()
    }
}

fn main() {
    println!("Hello, world!");
}
