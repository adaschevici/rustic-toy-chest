use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = do_some_io().await?;
    println!("Result: {}", result);
    Ok(())
}

async fn do_some_io() -> Result<String, Box<dyn Error>> {
    let path = "Cargo.toml";
    let mut file = File::open(path).await?;

    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    Ok(contents)
}
