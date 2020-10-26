use std::collections::HashMap;
use tokio::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::io;
use anyhow::Result;
use prost::Message;


#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await
        });
    }
}

//TODO: need to seperate the read stream and write stream
async fn process(socket: TcpStream) -> Result<()> {
    let (mut rx, mut wx) = tokio::io::split(socket);
    let mut resp = Vec::new();
    let mut buf = vec![0; 1024]; 
    loop {
        let n = rx.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        resp.append(&mut buf);
    } 
    println!("get resp: {:?}", String::from_utf8(resp));
    println!("{:?}", env!("OUT_DIR"));    
    Ok(())
}