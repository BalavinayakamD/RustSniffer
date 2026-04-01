use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;

use crate::protocols::{ipv4, ipv6};

pub fn handle_ethernet(packet: &[u8]) {
    if let Some(eth_packets) = EthernetPacket::new(packet) {
        match eth_packets.get_ethertype() {
            EtherTypes::Ipv4 => {
                // println!("IPv4 packet detected");
                ipv4::handle_ipv4(eth_packets.payload());
            }
            EtherTypes::Ipv6 => {
                // println!("IPv6 packet detected");
                ipv6::handle_ipv6(eth_packets.payload());
            }
            _ => { }
        }
    }
}
