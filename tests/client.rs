use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::test]
async fn clinet_test() {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let mut s = String::new();
    for _ in 0..10000 {
        s.push('1');
    }
    // Write some data.
    stream.write_all(s.as_bytes()).await.unwrap();
}
