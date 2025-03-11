use tokio;

mod cli;

#[tokio::main]
async fn main() {
    println!("******RustMan******");
    cli::fs().await;
}
