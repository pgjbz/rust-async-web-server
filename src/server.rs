use std::{error::Error, net::{SocketAddr, TcpListener}, io::{Read, Write}};

pub fn run() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr)?;
    println!("Linstening on http://{addr}");
    
    for stream in listener.incoming() {
        let mut buffer = [0; 1024];
        let mut stream = stream.unwrap();
        stream.read(&mut buffer).unwrap(); //block until receive
        println!("{}", String::from_utf8(Vec::from(buffer))?);
        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
        stream.write_all(b"Hello from server")?;
        break;
    }
    println!("Closing connection from {addr}");
    Ok(())
}