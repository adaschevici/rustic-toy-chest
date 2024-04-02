use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = do_some_io().await?;
    println!("Result: {}", result);

    // tasks
    let data = vec![1, 2, 3, 4, 5];
    let processed_data = process_data(data).await;
    println!("Processed data: {:?}", processed_data);
    Ok(())
}

async fn do_some_io() -> Result<String, Box<dyn Error>> {
    let path = "Cargo.toml";
    let mut file = File::open(path).await?;

    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    Ok(contents)
}

async fn process_data(data: Vec<i32>) -> Vec<i32> {
    let mut tasks = Vec::new();
    for num in data {
        let task = task::spawn(async move {
            let result = some_async_operation(num).await;
            result
        });
        tasks.push(task);
    }

    let mut processed_data = Vec::new();
    for task in tasks {
        let result = task.await.unwrap();
        processed_data.push(result);
    }

    processed_data
}

async fn some_async_operation(input: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    input + 1
}
