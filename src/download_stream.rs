use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Url;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::URL;

pub async fn download_large_file_stream() -> Result<()> {
    let saved_path = PathBuf::from("src");
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // To disable SSL verification
        .build()?;
    match client.get(URL).send().await {
        Ok(response) => {
            let total_size = response
                .content_length()
                .ok_or(format!("Failed to get content length from '{}'", &URL))
                .unwrap();

            // download chunks
            let file_name = file_name(response.url().clone());
            let fname = saved_path.join(file_name);
            let mut file = File::create(fname)
                .or(Err(format!("Failed to create file  ",)))
                .unwrap();
            let mut downloaded: u64 = 0;
            let mut stream = response.bytes_stream();

            while let Some(item) = stream.next().await {
                let chunk = item
                    .or(Err(format!("Error while downloading file")))
                    .unwrap();
                file.write_all(&chunk)
                    .or(Err(format!("Error while writing to file")))
                    .unwrap();

                let new = min(downloaded + (chunk.len() as u64), total_size);
                downloaded = new;
                println!("Downloaded: {}/{}", downloaded, total_size);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}

fn file_name(url: Url) -> String {
    url.path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin")
        .to_string()
}
