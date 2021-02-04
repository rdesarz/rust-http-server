use http_server::server::Server;
use http_server::tcp_connection::TcpServerConnection;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    // Create connection for the server
    let tcp_server_connection = TcpServerConnection::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        5666,
    ))
    .expect("Unable to initialize connection. Server shutdown");
    // Init Http server
    let http_server = Server::new(tcp_server_connection);
    http_server.run();
}
