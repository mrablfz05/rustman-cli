use clap::ArgMatches;
use console::style;
use reqwest;

use crate::utils::progress;

pub async fn execute(sub_m: &ArgMatches) {
    let url = sub_m.get_one::<String>("url").unwrap();

    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(content) => println!(
                "{}",
                style(format!("Response from '{}':\n{}", url, content)).cyan()
            ),
            Err(e) => eprintln!("Error reading response from '{}': {}", url, e),
        },
        Err(e) => eprintln!("Failed to fetch '{}': {}", url, e),
    }
}
