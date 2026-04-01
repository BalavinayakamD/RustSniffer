#[derive(Debug, Default)]
pub struct ParsedPacket {
    pub src_mac: Option<String>,
    pub dst_mac: Option<String>,

    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,

    pub protocol: Option<String>,

    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,

    pub payload: Vec<u8>,
}

