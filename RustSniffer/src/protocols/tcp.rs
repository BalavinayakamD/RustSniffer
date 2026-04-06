use pnet::packet::Packet;
// use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;

use crate::packet::ParsedPacket;

pub fn handle_tcp(packet: &[u8], parsed: &mut ParsedPacket) {
    if let Some(tcp) = TcpPacket::new(packet) {
        parsed.protocol = Some("TCP".to_string());
        parsed.src_port = Some(tcp.get_source());
        parsed.dst_port = Some(tcp.get_destination());
        parsed.payload = tcp.payload().to_vec();
        // println!(
        //     "[TCP] {}:{} → {}:{}",
        //     ip.get_source(),
        //     tcp.get_source(),
        //     ip.get_destination(),
        //     tcp.get_destination()
        // );
    }
}
