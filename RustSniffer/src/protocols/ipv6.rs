use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv6::Ipv6Packet;

use crate::{
    packet::ParsedPacket,
    protocols::{tcp, udp},
};

pub fn handle_ipv6(packet: &[u8], parsed: &mut ParsedPacket) {
    if let Some(ip) = Ipv6Packet::new(packet) {
        parsed.src_ip = Some(ip.get_source().to_string());
        parsed.dst_ip = Some(ip.get_destination().to_string());

        match ip.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp::handle_tcp(ip.payload(), parsed);
            }
            IpNextHeaderProtocols::Udp => {
                udp::handle_udp(ip.payload(), parsed);
            }
            _ => {
                parsed.protocol = Some("Other".to_string());
                parsed.payload = ip.payload().to_vec();
            }
        }
    }
}
