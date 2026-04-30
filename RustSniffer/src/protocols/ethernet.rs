use crate::packet::ParsedPacket;
use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

use crate::protocols::{ipv4, ipv6};

pub fn handle_ethernet(packet: &[u8]) -> Option<ParsedPacket> {
    if let Some(eth_packets) = EthernetPacket::new(packet) {
        let mut parsed = ParsedPacket::default();
        parsed.src_mac = Some(eth_packets.get_source().to_string());
        parsed.dst_mac = Some(eth_packets.get_destination().to_string());

        match eth_packets.get_ethertype() {
            EtherTypes::Ipv4 => {
                ipv4::handle_ipv4(eth_packets.payload() , &mut parsed);
            }
            EtherTypes::Ipv6 => {
                ipv6::handle_ipv6(eth_packets.payload(), &mut parsed);
            }
            _ => {}
        }
        return Some(parsed);
    }
    return None;
}
