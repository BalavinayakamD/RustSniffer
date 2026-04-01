use pnet::datalink::{self , Channel::Ethernet};
use crate::protocols::ethernet;

pub fn capture_packets(interface_name: &str) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
                    .into_iter()
                    .find(|iface| iface.name == interface_name)
                    .expect("Error: Could not find the specified interface");

    let mut config = datalink::Config::default();
    config.promiscuous =  true;

    let (_ , mut rx) = match datalink::channel(&interface , config) {
        Ok(Ethernet(_, rx)) => ( () , rx),
        Ok(_) => panic!("Error: Unhandles channel type"),
        Err(e) => panic!("Error: Failed to create datalink channel: {}", e),
    };

    println!("Capturing packets on interface: {}", interface_name);

    loop {
        match rx.next() {
            Ok(packet) => {
                ethernet::handle_ethernet(packet);
            },
            Err(e) => {
                eprintln!("Error: Failed to read packet: {}", e);
            }
        }
    }
}
