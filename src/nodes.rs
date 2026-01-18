#![allow(dead_code)]
use std::collections::HashMap;

use crate::{
    link::{Link, PortId},
    packet::{EthernetFrame, MacAddress},
};
pub type NodeId = u8;

pub trait NodeHandler {
    fn send_pkt(
        &self,
        dst_mac: &MacAddress,
        out_port: PortId,
    ) -> Option<NodeAction>;

    fn rcv_pkt(
        &self,
        frame: &EthernetFrame,
        port: PortId
    ) -> Option<NodeAction>;
}

#[derive(Debug, Clone)]
pub struct SimpleNode {
    pub id: NodeId,
    pub mac: MacAddress,
    pub port: PortId,
}

impl SimpleNode {
    pub fn new(id: NodeId, mac: MacAddress, port: PortId) -> Self {
        Self { id, mac, port }
    }
}

impl NodeHandler for SimpleNode {
    fn send_pkt(
        &self,
        dst_mac: &MacAddress,
        out_port: PortId,
    ) -> Option<NodeAction> {
        let frame = EthernetFrame::new(self.mac, dst_mac.clone(), 0x800, b"Hello".to_vec());
        Some(NodeAction::Send { from: self.id, port: out_port, frame })
    }

    fn rcv_pkt(
        &self,
        _frame: &EthernetFrame,
        _port: PortId
    ) -> Option<NodeAction> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub mac: MacAddress,
    pub ports: HashMap<PortId, Link>,
    mac_table: HashMap<PortId, MacAddress>,
}

pub enum NodeAction {
    Send {
        from: NodeId,
        port: PortId,
        frame: EthernetFrame,
    },
    Rcv {
        to: NodeId,
        port: PortId,
        frame: EthernetFrame,
    },
}

impl Node {
    pub fn new(id: NodeId, mac: MacAddress) -> Self {
        Self {
            id,
            mac,
            ports: HashMap::new(),
            mac_table: HashMap::new(),
        }
    }

    pub fn send_pkt(
        &self,
        port: PortId,
        dst_mac: &MacAddress,
        payload: Vec<u8>,
    ) -> Option<NodeAction> {
        let frame = EthernetFrame::new(self.mac, dst_mac.clone(), 0x0800, payload);
        let from = self.id;
        Some(NodeAction::Send { from, port, frame })
    }

    pub fn rcv_pkt(&self, _pkt: &EthernetFrame, _port: PortId) -> Option<NodeAction> {
        None
    }

    pub fn install_static_mac_address(&mut self, mac: &MacAddress, port: PortId) {
        self.mac_table.insert(port, mac.clone());
    }

    pub fn update_port(&mut self, port: PortId, link: &Link) {
        self.ports.insert(port, link.clone());
    }
}
