use futures::future::Either;
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

async fn task_three() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    "task_three".to_string()
}

async fn task_four() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "task_four".to_string()
}

async fn long_running_task_later() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    "long_running_task completed".to_string()
}

async fn network_request() -> Result<String, String> {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok("network_request".to_string())
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

    let result = select! {
        result = task_three() => {
            Either::Left(result)
        },
        result = task_four() => {
            Either::Right(result)
        }
    };
    match result {
        Either::Left(res) => {
            println!("task_three completed with: {}", res);
        }
        Either::Right(res) => {
            println!("task_four completed with: {}", res);
        }
    }

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    let long_running_task = long_running_task_later();
    let mut long_running_task = Box::pin(long_running_task);

    loop {
        select! {
            _ = interval.tick() => {
                println!("tick");
            }
            result = long_running_task.as_mut() => {
                println!("long_running_task completed: {:?}", result);
                break;
            }
        }
    }

    let timeout = tokio::time::timeout(std::time::Duration::from_secs(3), network_request());
    select! {
        _ = timeout => {
            println!("Request timed out");
        }
        result = network_request() => {
            match result {
                Ok(res) => {
                    println!("network_request completed with: {}", res);
                }
                Err(e) => {
                    println!("network_request timed out with error: {:?}", e);
                }
            }
        }
    }
}
