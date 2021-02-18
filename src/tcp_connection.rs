use crate::server::{Connection, ServerError};
use crate::thread_pool::ThreadPool;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct TcpServerConnection {
    pub listener: TcpListener,
    pool: ThreadPool,
}

impl TcpServerConnection {
    pub fn new(socket: SocketAddr) -> io::Result<TcpServerConnection> {
        let listener = TcpListener::bind(socket)?;
        Ok(TcpServerConnection {
            listener,
            pool: ThreadPool::new(4),
        })
    }
}

impl TcpServerConnection {
    fn handle_incoming_connection<T: Fn(&[u8]) -> Result<Vec<u8>, ServerError> + Send + Sync>(
        request_handler_callback: T,
        stream: &mut TcpStream,
    ) {
        let mut input_buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut input_buffer).unwrap();
        match (request_handler_callback)(&input_buffer) {
            Ok(message) => {
                if let Ok(_written_size) = stream.write(&message) {
                    stream.flush();
                }
            }
            Err(e) => {
                println!("Error when handling request: {:?}", e);
            }
        }
    }
}

impl Connection for TcpServerConnection {
    fn listen<T: 'static + Copy + Fn(&[u8]) -> Result<Vec<u8>, ServerError> + Send + Sync>(
        &self,
        request_handler_callback: T,
    ) {
        for connection in self.listener.incoming() {
            match connection {
                Ok(mut socket) => {
                    self.pool.execute(move || {
                        Self::handle_incoming_connection(&request_handler_callback, &mut socket);
                    });
                }
                Err(e) => println!("Error when getting client: {:?}", e),
            }
        }
    }
}
