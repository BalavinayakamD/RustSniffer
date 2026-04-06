use pnet::packet::Packet;
// use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;

use crate::packet::ParsedPacket;

pub fn handle_udp(packet: &[u8], parsed: &mut ParsedPacket) {
    if let Some(udp) = UdpPacket::new(packet) {
        parsed.protocol = Some("UDP".to_string());
        parsed.src_port = Some(udp.get_source());
        parsed.dst_port = Some(udp.get_destination());
        parsed.payload = udp.payload().to_vec();

        // println!(
        //     "[UDP] {}:{} → {}:{}",
        //     ip.get_source(),
        //     udp.get_source(),
        //     ip.get_destination(),
        //     udp.get_destination()
        // );
    }
}
