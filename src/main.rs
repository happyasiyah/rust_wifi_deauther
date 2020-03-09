use mio::net::{UdpSocket};
use pnet::datalink;

fn get_network_interfaces() -> Vec<datalink::NetworkInterface> {
    return datalink::interfaces();
}

fn send_authentication() {
    
}

fn bind_socket() -> Result<(), std::net::AddrParseError> {
    let socket = UdpSocket::bind("127.0.0.1:0".parse()?);
    return Ok(());
}

fn deauth() {
    
}

fn main() {

}
