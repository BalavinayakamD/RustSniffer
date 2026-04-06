use crate::protocols::ethernet;
use pnet::datalink::{self, Channel::Ethernet};

pub fn capture_packets(interface_name: &str) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Error: Could not find the specified interface");

    let mut config = datalink::Config::default();
    config.promiscuous = true;

    let (_, mut rx) = match datalink::channel(&interface, config) {
        Ok(Ethernet(_, rx)) => ((), rx),
        Ok(_) => panic!("Error: Unhandles channel type"),
        Err(e) => panic!("Error: Failed to create datalink channel: {}", e),
    };

    println!("Capturing packets on interface: {}", interface_name);

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(parsed) = ethernet::handle_ethernet(packet) {
                    println!(
                        "SRC_MAC: {:?} | DST_MAC: {:?} | SRC_IP: {:?} | DST_IP: {:?} | PROTO: {:?} | SRC_PORT: {:?} | DST_PORT: {:?}",
                        parsed.src_mac.as_deref().unwrap_or("-"),
                        parsed.dst_mac.as_deref().unwrap_or("-"),
                        parsed.src_ip.as_deref().unwrap_or("-"),
                        parsed.dst_ip.as_deref().unwrap_or("-"),
                        parsed.protocol.as_deref().unwrap_or("-"),
                        parsed.src_port,
                        parsed.dst_port
                    );
                }
            }
            Err(e) => {
                eprintln!("Error: Failed to read packet: {}", e);
            }
        }
    }
}
