use crate::protocols::ethernet;
use crate::handlers::{format_and_print, OutputFormat};
use pnet::datalink::{self, Channel::Ethernet};
use std::time::Duration;

pub fn capture_packets(
    interface_name: &str,
    output: OutputFormat,
    max_count: u64,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    filter: Option<String>,
) {
    let interfaces = datalink::interfaces();
    let interface = match interfaces.into_iter().find(|iface| iface.name == interface_name) {
        Some(i) => i,
        None => {
            eprintln!("Error: Could not find the specified interface: {}", interface_name);
            return;
        }
    };

    let mut config = datalink::Config::default();
    config.promiscuous = true;
    config.read_timeout = Some(Duration::from_millis(100));

    let (_, mut rx) = match datalink::channel(&interface, config) {
        Ok(Ethernet(_, rx)) => ((), rx),
        Ok(_) => {
            eprintln!("Error: Unhandled channel type");
            return;
        }
        Err(e) => {
            eprintln!("Error: Failed to create datalink channel: {}", e);
            return;
        }
    };

    eprintln!("Capturing packets on interface: {}", interface_name);
    if let Some(ref f) = filter {
        eprintln!("[INFO] Simple filter: {}", f);
    }

    let mut seen: u64 = 0;
    let mut matched: u64 = 0;
    let mut protocol_counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    let mut port_counts: std::collections::HashMap<u16, u64> = std::collections::HashMap::new();

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        match rx.next() {
            Ok(packet) => {
                seen += 1;

                if let Some(parsed) = ethernet::handle_ethernet(packet) {
                    // Apply simple filter if provided
                    let passes = packet_matches_filter(&parsed, &filter);
                    
                    if passes {
                        // Update protocol count
                        if let Some(proto) = parsed.protocol.as_deref() {
                            *protocol_counts.entry(proto.to_string()).or_insert(0) += 1;
                        }

                        // Update port counts
                        if let Some(sp) = parsed.src_port {
                            *port_counts.entry(sp).or_insert(0) += 1;
                        }
                        if let Some(dp) = parsed.dst_port {
                            *port_counts.entry(dp).or_insert(0) += 1;
                        }

                        format_and_print(&parsed, output);
                        matched += 1;

                        if max_count > 0 && matched >= max_count {
                            eprintln!("Captured {} matching packets, exiting.", matched);
                            running.store(false, std::sync::atomic::Ordering::SeqCst);
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                // Timeout is normal, just continue checking running flag
            }
        }
    }

    // Print summary
    eprintln!("Exiting capture loop.");
    println!("Summary:");
    println!("  Total packets seen: {}", seen);
    println!("  Packets matched/printed: {}", matched);

    if !protocol_counts.is_empty() {
        println!("  Protocol counts:");
        let mut protos: Vec<_> = protocol_counts.into_iter().collect();
        protos.sort_by(|a, b| b.1.cmp(&a.1));
        for (p, c) in protos.iter().take(10) {
            println!("    {}: {}", p, c);
        }
    }

    if !port_counts.is_empty() {
        println!("  Top ports:");
        let mut ports: Vec<_> = port_counts.into_iter().collect();
        ports.sort_by(|a, b| b.1.cmp(&a.1));
        for (p, c) in ports.iter().take(10) {
            println!("    {}: {}", p, c);
        }
    }
}

fn packet_matches_filter(p: &crate::packet::ParsedPacket, filter: &Option<String>) -> bool {
    match filter {
        None => true,
        Some(f) => {
            let f = f.trim().to_lowercase();
            if f == "tcp" {
                return p.protocol.as_deref().map(|s| s.to_lowercase().contains("tcp")).unwrap_or(false);
            }
            if f == "udp" {
                return p.protocol.as_deref().map(|s| s.to_lowercase().contains("udp")).unwrap_or(false);
            }
            if f == "ip" || f == "ipv4" || f == "ipv6" {
                return p.src_ip.is_some() || p.dst_ip.is_some();
            }
            if f.starts_with("port ") {
                if let Ok(port) = f[5..].trim().parse::<u16>() {
                    return p.src_port == Some(port) || p.dst_port == Some(port);
                }
            }

            // fallback: substring match across several fields
            let needle = f;
            if p.src_ip.as_deref().map(|s| s.contains(&needle)).unwrap_or(false)
                || p.dst_ip.as_deref().map(|s| s.contains(&needle)).unwrap_or(false)
                || p.src_mac.as_deref().map(|s| s.contains(&needle)).unwrap_or(false)
                || p.dst_mac.as_deref().map(|s| s.contains(&needle)).unwrap_or(false)
                || p.protocol.as_deref().map(|s| s.to_lowercase().contains(&needle)).unwrap_or(false)
            {
                return true;
            }

            false
        }
    }
}

