use crate::http::server::{Connection, ServerError};
use crate::thread::pool::ThreadPool;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};

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
    fn handle_incoming_connection<
        Callback: Fn(&[u8]) -> Result<Vec<u8>, ServerError> + Send + Sync,
        Stream: Read + Write,
    >(
        request_handler_callback: Callback,
        stream: &mut Stream,
    ) {
        let mut input_buffer: [u8; 1024] = [0; 1024];
        match stream.read(&mut input_buffer) {
            Ok(_) => {
                match (request_handler_callback)(&input_buffer)
                    .map(|message| stream.write(&message))
                    .map(|_| stream.flush())
                {
                    Ok(_) => println!("Request was succesfully handled"),
                    Err(e) => println!("Error when handling request: {:?}", e),
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStream {
        input_data: Vec<u8>,
        output_data: Vec<u8>,
        was_flushed: bool,
    }

    impl Write for TestStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output_data = Vec::from(buf);
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.was_flushed = true;
            Ok(())
        }
    }

    impl Read for TestStream {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            for (buf_byte, data) in buf.iter_mut().zip(self.input_data.iter()) {
                *buf_byte = *data
            }
            Ok(0)
        }
    }

    #[test]
    fn success_request_handling() {
        let mut stream = TestStream {
            input_data: String::from("input").as_bytes().to_vec(),
            output_data: vec![],
            was_flushed: false,
        };

        TcpServerConnection::handle_incoming_connection(
            |_| Ok(String::from("output").as_bytes().to_vec()),
            &mut stream,
        );

        assert_eq!(
            stream.output_data,
            String::from("output").as_bytes().to_vec()
        );
        assert!(stream.was_flushed,);
    }

    #[test]
    fn failure_request_handling() {
        let mut stream = TestStream {
            input_data: String::from("input").as_bytes().to_vec(),
            output_data: vec![],
            was_flushed: false,
        };

        TcpServerConnection::handle_incoming_connection(
            |_| Err(ServerError::new("Test error")),
            &mut stream,
        );

        assert_eq!(stream.output_data, String::from("").as_bytes().to_vec());
        assert!(!stream.was_flushed,);
    }
}
