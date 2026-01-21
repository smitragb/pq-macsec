#![allow(dead_code)]

use crate::{
    link::PortId,
    packet::{EthernetFrame, MacAddress},
};
pub type NodeId = u8;

pub trait NodeHandler {
    fn send_pkt(&self, dst_mac: &MacAddress, out_port: PortId) -> Option<NodeAction>;
    fn rcv_pkt(&self, frame: &EthernetFrame, port: PortId) -> Option<NodeAction>;
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
    fn send_pkt(&self, dst_mac: &MacAddress, out_port: PortId) -> Option<NodeAction> {
        let frame = EthernetFrame::new(self.mac, dst_mac.clone(), 0x800, b"Hello".to_vec());
        Some(NodeAction::Send {
            from: self.id,
            port: out_port,
            frame,
        })
    }

    fn rcv_pkt(&self, _frame: &EthernetFrame, _port: PortId) -> Option<NodeAction> {
        None
    }
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
