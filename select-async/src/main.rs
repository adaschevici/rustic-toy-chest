use tokio::{self, task};

async fn task_one() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    "task_one".to_string()
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
