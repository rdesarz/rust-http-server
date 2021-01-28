use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod http_connection {
    use std::net::Ipv4Addr;

    struct Socket {
        ip: Ipv4Addr,
        port: u16,
    }

    impl Socket {
        fn new(ip: Ipv4Addr, port: u16) -> Socket {
            Socket { ip, port }
        }
    }

    impl ToString for Socket {
        fn to_string(&self) -> String {
            let mut string = self.ip.to_string();
            string.push_str(":");
            string.push_str(&*self.port.to_string());
            string
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn print_good_value() {
            let socket = Socket::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
            assert_eq!(socket.to_string(), "127.0.0.1:8080");
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    for bytes in buffer.iter() {
        println!("{}", bytes);
    }
    stream.write(&buffer);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!(" Connection error");
            }
        }
    }
}
