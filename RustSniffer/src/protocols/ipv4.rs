use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;

use crate::protocols::{tcp, udp};

pub fn handle_ipv4(packet: &[u8]) {
    if let Some(ip) = Ipv4Packet::new(packet) {
        match ip.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp::handle_tcp(ip.payload(), &ip);
            }
            IpNextHeaderProtocols::Udp => {
                udp::handle_udp(ip.payload(), &ip);
            }
            _ => {}
        }
    }
}