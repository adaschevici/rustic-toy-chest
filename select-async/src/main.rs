use tokio::select;

async fn task_one() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    "task_one".to_string()
}

async fn task_two() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "task_two".to_string()
}

async fn long_running_task() {
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    println!("long_running_task completed");
}

async fn user_input() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    "User input".to_string()
}

#[tokio::main]
async fn main() {
    select! {
        res = task_one() => {
            println!("task_one completed with: {}", res);
        }
        res = task_two() => {
            println!("task_two completed with: {}", res);
        }
    }

    select! {
        _ = long_running_task() => {
            println!("long_running_task completed");
        }
        input = user_input() => {
            println!("User input: {}", input);
        }
    }
}
