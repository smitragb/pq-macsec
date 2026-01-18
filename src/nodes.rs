#![allow(dead_code)]
use std::collections::HashMap;

use crate::{link::PortId, packet::{EthernetFrame, MacAddress}};
pub type NodeId = u8;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub mac: MacAddress,
    mac_table: HashMap<PortId, MacAddress>
}

pub enum NodeAction {
    Send { from: NodeId, frame: EthernetFrame },
    Rcv { to: NodeId, frame: EthernetFrame },
}

impl Node {
    pub fn new(id: NodeId, mac: MacAddress) -> Self {
        Self { 
            id,
            mac,
            mac_table: HashMap::new()
        }
    }

    pub fn send_pkt(&self, dst_mac: &MacAddress, payload: Vec<u8>) -> Option<NodeAction> {
        let frame = EthernetFrame::new(self.mac, dst_mac.clone(), 0x0800, payload);
        let from = self.id;
        Some(NodeAction::Send { from, frame })
    }

    pub fn rcv_pkt(&self, _pkt: &EthernetFrame) -> Option<NodeAction> {
        None
    }

    pub fn install_static_mac_address(&mut self, mac: MacAddress, port: PortId) {
        self.mac_table.insert(port, mac);
    }
}
