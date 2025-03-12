use clap::ArgMatches;
use console::style;
use indicatif::ProgressStyle;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::utils::progress;

pub async fn execute(sub_m: &ArgMatches) {
    let url = sub_m.get_one::<String>("url").unwrap();
    let client = Client::new();

    match client.get(url).send().await {
        Ok(mut response) => {
            let total_size = response.content_length().unwrap_or(100);

            let pb = progress::new(total_size);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            let mut file = File::create("downloaded_file.txt")
                .await
                .expect("Failed to create file");

            let mut downloaded: u64 = 0;

            while let Some(chunk) = response.chunk().await.unwrap() {
                file.write_all(&chunk).await.expect("Failed to write chunk");
                downloaded += chunk.len() as u64;
                pb.set_position(downloaded);
            }

            pb.finish_with_message("Download complete!");
            println!("{}", style("File downloaded successfully!").green());
        }
        Err(e) => eprintln!("Failed to fetch '{}': {}", url, e),
    }
}
