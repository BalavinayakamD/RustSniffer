use pnet::packet::tcp::TcpPacket;
// use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;

pub fn handle_tcp(packet: &[u8], ip: &Ipv4Packet) {
    if let Some(tcp) = TcpPacket::new(packet) {
        println!(
            "[TCP] {}:{} → {}:{}",
            ip.get_source(),
            tcp.get_source(),
            ip.get_destination(),
            tcp.get_destination()
        );
    }
}
