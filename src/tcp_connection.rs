use crate::server::Connection;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};

pub struct TcpServerConnection {
    pub listener: TcpListener,
}

impl TcpServerConnection {
    pub fn new(socket: SocketAddr) -> io::Result<TcpServerConnection> {
        let listener = TcpListener::bind(socket)?;
        Ok(TcpServerConnection { listener })
    }
}

impl Connection for TcpServerConnection {
    fn listen<T: Fn(&str) -> Result<String, std::io::Error>>(&self, request_handler_callback: T) {
        for connection in self.listener.incoming() {
            match connection {
                Ok(mut socket) => {
                    let mut input_buffer: [u8; 1024] = [0; 1024];
                    socket.read(&mut input_buffer).unwrap();
                    match (request_handler_callback)(std::str::from_utf8(&input_buffer).unwrap()) {
                        Ok(message) => {
                            if let Ok(_written_size) = socket.write(message.as_bytes()) {
                                socket.flush();
                            }
                        }
                        Err(e) => {
                            println!("Error when handling request: {:?}", e);
                        }
                    }
                }
                Err(e) => println!("Error when getting client: {:?}", e),
            }
        }
    }
}
