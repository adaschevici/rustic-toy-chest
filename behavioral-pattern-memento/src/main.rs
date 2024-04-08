use inquire::Select;

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

struct Editor {
    text: String,
    caretaker: CaretakerEditor,
}

impl Editor {
    fn new() -> Editor {
        Editor {
            text: String::new(),
            caretaker: CaretakerEditor::new(),
        }
    }

    fn write(&mut self, content: &str) {
        self.caretaker.add_memento(Memento::new(self.text.clone()));
        self.text.push_str(content);
    }

    fn undo(&mut self) {
        if let Some(memento) = self.caretaker.undo() {
            self.text = memento.get_state().to_string();
        }
    }

    fn redo(&mut self) {
        if let Some(memento) = self.caretaker.redo() {
            self.text = memento.get_state().to_string();
        }
    }
}

struct CaretakerEditor {
    mementos: Vec<Memento>,
    index: usize,
}

impl CaretakerEditor {
    fn new() -> CaretakerEditor {
        CaretakerEditor {
            mementos: Vec::new(),
            index: 0,
        }
    }

    fn add_memento(&mut self, memento: Memento) {
        self.mementos.push(memento);
    }

    fn undo(&mut self) -> Option<Memento> {
        if !self.mementos.is_empty() {
            self.mementos.pop()
        } else {
            None
        }
    }

    fn redo(&mut self) -> Option<&Memento> {
        if self.index < self.mementos.len() - 1 {
            self.index += 1;
            Some(&self.mementos[self.index])
        } else {
            None
        }
    }
}

struct Document {
    content: String,
}

impl Document {
    fn new(content: String) -> Self {
        Document { content }
    }

    fn create_snapshot(&self) -> Memento {
        Memento::new(self.content.clone())
    }

    fn restore_snapshot(&mut self, memento: &Memento) {
        self.content = memento.get_state().to_string();
    }
}

fn run_basic_memento_usecase() {
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

// this needs some tlc to get it to work properly. This is due to the fact that the undo is
// destructive and redo should keep track of the undos
fn run_text_editor_usecase() {
    let mut editor = Editor::new();

    editor.write("Hello, ");
    editor.write("World!");

    println!("Current text: {}", editor.text);

    editor.undo();
    println!("Current text: {}", editor.text);

    editor.redo();
    println!("Current text: {}", editor.text);
}

fn run_document_snapshots_usecase() {
    let mut document = Document::new("Initial content".to_string());
    let mut caretaker = Caretaker::new();

    caretaker.add_memento(document.create_snapshot());

    document.content = "Content 1".to_string();
    caretaker.add_memento(document.create_snapshot());

    document.content = "Content 2".to_string();
    caretaker.add_memento(document.create_snapshot());

    if let Some(memento) = caretaker.mementos.get(1) {
        document.restore_snapshot(memento);
        println!("Current content: {}", document.content);
    }
}

fn main() {
    let actions = vec!["basic", "editor", "document_snapshots"];
    let actions_map: std::collections::HashMap<&str, fn()> = [
        ("basic", run_basic_memento_usecase as fn()),
        ("editor", run_text_editor_usecase as fn()),
        ("document_snapshots", run_document_snapshots_usecase as fn()),
    ]
    .into_iter()
    .collect();

    let selected_action = Select::new("Choose an action:", actions).prompt();

    match selected_action {
        Ok(selected) => {
            if let Some(&action) = actions_map.get(selected) {
                action(); // Execute the selected action
            }
        }
        Err(_) => println!("Error or user aborted prompt."),
    }
}
