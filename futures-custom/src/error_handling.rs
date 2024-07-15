use std::io;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn read_file(path: &str) -> Result<(), io::Error> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("{}", contents);
    Ok(())
}

pub async fn read_file_with_error_handling() {
    match read_file("nonexistent").await {
        Ok(_) => println!("File read successfully"),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}
