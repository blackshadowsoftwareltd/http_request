use download_stream::download_large_file_stream;
pub mod download;
pub mod download_bytes;
pub mod download_stream;
// use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
// use std::net::TcpStream;

#[tokio::main]
async fn main() {
    // let client = reqwest::Client::new();
    // let connector = SslConnector::builder(SslMethod::tls())
    //     .unwrap()
    //     .verify_mode(SslVerifyMode::NONE)
    //     .build();

    // let stream = TcpStream::connect("127.0.0.1:49154").unwrap();
    // let ssl_stream = connector.connect("127.0.0.1", stream).unwrap();

    // download_file().await
    download_large_file_stream().await.unwrap()
}

pub const URL: &str = &"http://127.0.0.1:49152/files/video_files.zip";
