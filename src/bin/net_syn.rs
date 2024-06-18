use pnet::packet::ip::IpNextHeaderProtocols;
/**
* 演示：给127.0.0.1:1025 发送第一个tcp syn 包
nc -l 1025
sudo tcpdump -i lo0 port 1025
telnet localhost 1025
*/
// use pnet::packet::tcp::TcpPacket;
use pnet::packet::tcp::TcpFlags;
// use pnet::packet::ipv4::Ipv4Packet;
// use pnet::packet::Packet;
use pnet::packet::tcp::MutableTcpPacket;
use pnet::transport::{self, TransportChannelType, TransportProtocol};
use std::net::{IpAddr, Ipv4Addr};

// sudo cargo run --bin net_syn
fn main() {
    let protocol =
        TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp));
    let (mut tx, _) = transport::transport_channel(4096, protocol).unwrap();

    // let ipv4_source: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let ipv4_destination = Ipv4Addr::new(127, 0, 0, 1);
    let source_port = 12345;
    let destination_port = 1025;

    let mut tcp_buffer = [0u8; 20];
    let mut tcp_header = MutableTcpPacket::new(&mut tcp_buffer).unwrap();

    tcp_header.set_source(source_port);
    tcp_header.set_destination(destination_port);
    tcp_header.set_sequence(0);
    tcp_header.set_data_offset(5);//which means the TCP header is 5*4=20 bytes long, as each unit represents 4 bytes
    tcp_header.set_flags(TcpFlags::SYN);

    tx.send_to(tcp_header, IpAddr::V4(ipv4_destination))
        .unwrap();
}
