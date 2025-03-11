use tokio;
use console::style;

mod cli;

#[tokio::main]
async fn main() {
    println!("{}", style(format!("******RustMan******")).black().bold());
    cli::fs().await;
}
