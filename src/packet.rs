pub type MacAddress = [u8; 6];

#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub src_mac: MacAddress,
    pub dst_mac: MacAddress,
    pub ethertype: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(
        src_mac: MacAddress, 
        dst_mac: MacAddress, 
        ethertype: u16, 
        payload: Vec<u8>
    ) -> Self {
        Self {
            src_mac,
            dst_mac,
            ethertype,
            payload,
        }
    }
}
