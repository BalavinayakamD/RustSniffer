use crate::packet::ParsedPacket;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
	Text,
	Json,
}

impl OutputFormat {
	pub fn from_str(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"json" => OutputFormat::Json,
			_ => OutputFormat::Text,
		}
	}
}

#[derive(Serialize)]
struct SerializablePacket<'a> {
	src_mac: Option<&'a str>,
	dst_mac: Option<&'a str>,
	src_ip: Option<&'a str>,
	dst_ip: Option<&'a str>,
	protocol: Option<&'a str>,
	src_port: Option<u16>,
	dst_port: Option<u16>,
}

pub fn format_and_print(p: &ParsedPacket, format: OutputFormat) {
	match format {
		OutputFormat::Text => {
			println!(
				"SRC_MAC: {:?} | DST_MAC: {:?} | SRC_IP: {:?} | DST_IP: {:?} | PROTO: {:?} | SRC_PORT: {:?} | DST_PORT: {:?}",
				p.src_mac.as_deref().unwrap_or("-"),
				p.dst_mac.as_deref().unwrap_or("-"),
				p.src_ip.as_deref().unwrap_or("-"),
				p.dst_ip.as_deref().unwrap_or("-"),
				p.protocol.as_deref().unwrap_or("-"),
				p.src_port,
				p.dst_port
			);
		}
		OutputFormat::Json => {
			let sp = SerializablePacket {
				src_mac: p.src_mac.as_deref(),
				dst_mac: p.dst_mac.as_deref(),
				src_ip: p.src_ip.as_deref(),
				dst_ip: p.dst_ip.as_deref(),
				protocol: p.protocol.as_deref(),
				src_port: p.src_port,
				dst_port: p.dst_port,
			};
			if let Ok(s) = serde_json::to_string(&sp) {
				println!("{}", s);
			}
		}
	}
}

