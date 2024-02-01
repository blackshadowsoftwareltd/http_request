use download::download_file;
pub mod download;
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

    download_file().await
}
