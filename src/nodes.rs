#![allow(dead_code)]
use crate::packet::{EthernetFrame, MacAddress};
pub type NodeId = u8;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub id: NodeId,
    pub mac: MacAddress,
}

pub enum NodeAction {
    Send { from: Node, frame: EthernetFrame },
    Rcv { to: Node, frame: EthernetFrame },
}

impl Node {
    pub fn new(id: NodeId, mac: MacAddress) -> Self {
        Self { id, mac }
    }

    pub fn send_pkt(&self, dst_mac: MacAddress, payload: Vec<u8>) -> Option<NodeAction> {
        let frame = EthernetFrame::new(self.mac, dst_mac, 0x0800, payload);
        let from = self.clone();
        Some(NodeAction::Send { from, frame })
    }

    pub fn rcv_pkt(&self, pkt: &EthernetFrame) -> Option<NodeAction> {
        let src = hex::encode(pkt.src_mac);
        let dst = hex::encode(pkt.dst_mac);
        println!(
            "EthernetFrame received: src: 0x{}, dst: 0x{}, ethertype: {:#x}, payload: {:#?}",
            src,
            dst,
            pkt.ethertype,
            String::from_utf8(pkt.payload.clone()).unwrap()
        );
        None
    }
}
