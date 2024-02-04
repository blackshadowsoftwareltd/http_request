use anyhow::{bail, Result};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use std::str::FromStr;
use std::{fs::File, path::PathBuf};
const CHUNK_SIZE: u32 = 10240;

pub async fn download_large_file() -> Result<()> {
    let saved_path = PathBuf::from("src");
    let url = "http://127.0.0.1:49152/files/addr.rs";

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true) // To disable SSL verification
        .build()
        .unwrap();

    match client.head(url).send() {
        Ok(response) => {
            let length = file_length(response.headers())?;

            let mut output_file = File::create(saved_path)?;

            // let file_name = file_name(response.url().clone());
            println!("starting download...");

            for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
                print!("range: {:?}", range);
                let mut response = client.get(url).header(RANGE, range).send()?;

                let status = response.status();
                if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
                    bail!("Unexpected server response: {}", status)
                }
                std::io::copy(&mut response, &mut output_file)?;
            }
            let content = response.text()?;
            std::io::copy(&mut content.as_bytes(), &mut output_file)?;

            println!("Finished with success!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}

fn file_length(headers: &HeaderMap) -> Result<u64> {
    let length = headers
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")
        .unwrap();
    let length = u64::from_str(length.to_str()?)
        .map_err(|_| "invalid Content-Length header")
        .unwrap();
    Ok(length)
}

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
        if buffer_size == 0 {
            bail!("invalid buffer_size, give a value greater than zero.");
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .expect("string provided by format!"),
            )
        }
    }
}
