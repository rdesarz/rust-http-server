use crate::http::server::{Connection, ServerError};
use crate::thread::pool::ThreadPool;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

/// TCP connection implementation to handle HTTP request
pub struct TcpServerConnection {
    listener: TcpListener,
    pool: ThreadPool,
}

impl TcpServerConnection {
    /// Creates a new [`TcpServerConnection`]. Connection uses a thread pool with four threads.
    /// Returns std::io::Error if connection was not able to connect to provided socket.
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
        match stream.read(&mut input_buffer) {
            Ok(_) => {
                match (request_handler_callback)(&input_buffer)
                    .map(|message| stream.write(&message))
                    .map(|_| stream.flush())
                {
                    Ok(_) => println!("Request was succesfully handled"),
                    Err(e) => println!("{:?}", e),
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
}

impl Connection for TcpServerConnection {
    /// Loop over TCP connection and handle incoming requests using the provided callback.
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
