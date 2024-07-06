use async_trait::async_trait;

#[async_trait]
trait Component {
    async fn operation(&self) -> String;
}

struct ConcreteComponent;

#[async_trait]
impl Component for ConcreteComponent {
    async fn operation(&self) -> String {
        "ConcreteComponent".to_string()
    }
}

struct Decorator<T: Component + Send + Sync> {
    component: T,
}

impl<T: Component + Send + Sync> Decorator<T> {
    async fn new(component: T) -> Self {
        Decorator { component }
    }
}

#[async_trait]
impl<T: Component + Send + Sync> Component for Decorator<T> {
    async fn operation(&self) -> String {
        format!("Decorator({})", self.component.operation().await)
    }
}
struct MoodDecorator<T: Component + Send + Sync> {
    component: T,
    mood: String,
}
impl<T: Component + Send + Sync> MoodDecorator<T> {
    async fn new(component: T, mood: String) -> Self {
        MoodDecorator { component, mood }
    }
}
#[async_trait]
impl<T: Component + Send + Sync> Component for MoodDecorator<T> {
    async fn operation(&self) -> String {
        format!(
            "{} and is feeling {}",
            self.component.operation().await,
            self.mood
        )
    }
}
pub async fn run_decorator() {
    let animal = ConcreteComponent;
    let happy_animal = MoodDecorator::new(animal, "happy".to_string()).await;
    println!("The animal is now: {}", happy_animal.operation().await);
}
