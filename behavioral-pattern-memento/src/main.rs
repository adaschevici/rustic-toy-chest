struct Originator {
    state: String,
}

// This is the object that will be saved and restored
impl Originator {
    fn new(state: String) -> Originator {
        Originator { state }
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
        self.state = memento.get_state().clone();
    }
}

// This is the object that will wrap the state of the Originator
#[derive(Clone)]
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
    let mut originator = Originator::new("Initial state".to_string());
    let mut caretaker = Caretaker::new();

    caretaker.add_memento(originator.save_to_memento());

    originator.set_state("State 1".to_string());
    caretaker.add_memento(originator.save_to_memento());

    originator.set_state("State 2".to_string());
    caretaker.add_memento(originator.save_to_memento());

    if let Some(memento) = caretaker.mementos.get(1) {
        originator.restore_from_memento(memento.clone());
        println!("Current state: {}", originator.state);
    }
}
