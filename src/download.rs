use reqwest::Url;
use std::io::copy;
use std::{fs::File, path::PathBuf};

pub async fn download_large_file() {}

pub async fn download_file() {
    let saved_path = PathBuf::from("src");
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // To disable SSL verification
        .build()
        .unwrap();

    match client
        .get("http://127.0.0.1:49152/files/addr.rs")
        .send()
        .await
    {
        Ok(response) => {
            let file_name = file_name(response.url().clone());
            let mut dest = {
                println!("file_name: {}", file_name);

                println!("file to download: '{}'", file_name);
                let fname = saved_path.join(file_name);
                println!("will be located under: '{:?}'", fname);
                File::create(fname).unwrap()
            };
            let content = response.text().await.unwrap();
            copy(&mut content.as_bytes(), &mut dest).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn file_name(url: Url) -> String {
    url.path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin")
        .to_string()
}
