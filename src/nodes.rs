#![allow(dead_code)]

use crate::packet::EthernetFrame;
pub type NodeId = u8;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub id: NodeId,
    pub mac: [u8; 16],
}

pub enum NodeAction {
    Send { frame: EthernetFrame, },
    Rcv { frame: EthernetFrame, }
}

impl Node {
    pub fn new(id: NodeId, mac: [u8; 16]) -> Self {
        Self { id, mac }
    }

    pub fn send_pkt(&self, dst_mac: [u8; 16], payload: Vec<u8>) -> Option<NodeAction> {
        let frame = EthernetFrame::new(self.mac, dst_mac, 0x0800, payload);
        Some(NodeAction::Send { frame })
    }

    pub fn rcv_pkt(&self, pkt: &EthernetFrame) -> Option<NodeAction> {
        println!("Received pkt: {:#?}", pkt); 
        None
    }
}
