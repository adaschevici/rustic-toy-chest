trait Component {
    fn operation(&self) -> String;
}

struct ConcreteComponent;
impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        "ConcreteComponent".to_string()
    }
}

struct Decorator<T: Component> {
    component: T,
}

impl<T: Component> Decorator<T> {
    fn new(component: T) -> Self {
        Decorator { component }
    }
}

impl<T: Component> Component for Decorator<T> {
    fn operation(&self) -> String {
        format!("Decorator({})", self.component.operation())
    }
}
struct MoodDecorator<T: Component> {
    component: T,
    mood: String,
}
impl<T: Component> MoodDecorator<T> {
    fn new(component: T, mood: String) -> Self {
        MoodDecorator { component, mood }
    }
}
impl<T: Component> Component for MoodDecorator<T> {
    fn operation(&self) -> String {
        format!(
            "{} and is feeling {}",
            self.component.operation(),
            self.mood
        )
    }
}
pub async fn run_decorator() {
    let animal = ConcreteComponent;
    let happy_animal = MoodDecorator::new(animal, "happy".to_string());
    println!("The animal is now: {}", happy_animal.operation());
}
