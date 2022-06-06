use std::{error::Error, net::SocketAddr};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;
    println!("Linstening on http://{addr}");

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(err) = handle_connection(stream).await {
                eprintln!("Error handle connection {}", err)
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let read = stream.read(&mut buffer).await?;
    let mut vec = Vec::new();
    vec.extend_from_slice(&buffer[..read]); 
    println!("{}", String::from_utf8(vec)?);
    write_response(stream).await?;

    Ok(())
}

async fn write_response(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    stream.write_all(b"HTTP/1.1 200 OK\r\n").await?;
    stream.write_all(b"\r\n").await?;
    stream.write_all(b"Hello from plaque!\n").await?;
    Ok(())
}
