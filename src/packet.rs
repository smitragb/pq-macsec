#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EthernetFrame {
    pub src_mac: [u8; 16],
    pub dst_mac: [u8; 16],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(
        src_mac: [u8; 16], 
        dst_mac: [u8; 16], 
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
