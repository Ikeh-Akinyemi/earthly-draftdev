use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("example.com:80").await?;

    let request = "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
    stream.write_all(request.as_bytes()).await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    println!("{}", String::from_utf8_lossy(&response));

    Ok(())
}
