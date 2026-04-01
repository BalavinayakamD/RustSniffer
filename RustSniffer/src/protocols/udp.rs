use pnet::packet::udp::UdpPacket;
use pnet::packet::ipv4::Ipv4Packet;

pub fn handle_udp(packet: &[u8], ip: &Ipv4Packet) {
    if let Some(udp) = UdpPacket::new(packet) {
        println!(
            "[UDP] {}:{} → {}:{}",
            ip.get_source(),
            udp.get_source(),
            ip.get_destination(),
            udp.get_destination()
        );
    }
}