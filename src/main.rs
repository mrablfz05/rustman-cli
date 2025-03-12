use console::style;
use tokio;

mod cli;

#[tokio::main]
async fn main() {
    println!("{}", style(format!("******RustMan******")).black().bold());
    cli::fs().await;
}
