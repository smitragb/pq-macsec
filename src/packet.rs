#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EthernetFrame {
    src_mac: [u8; 16],
    dst_mac: [u8; 16],
    ethertype: u16,
    payload: Vec<u8>,
}
