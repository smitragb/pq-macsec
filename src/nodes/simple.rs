use tracing::info;

use crate::{
    link::{LinkEndId, PortId},
    nodes::{NodeAction, NodeHandler, NodeId},
    packet::{EthernetFrame, MacAddress},
};

#[derive(Clone)]
pub struct SimpleNode {
    pub id: NodeId,
    pub mac: MacAddress,
    pub port: PortId,
}

impl SimpleNode {
    pub fn new(id: NodeId, mac: MacAddress, port: PortId) -> Self {
        Self { id, mac, port }
    }

    pub fn get_link_id(&self) -> LinkEndId {
        (self.id, self.port)
    }
}

impl NodeHandler for SimpleNode {
    fn send_pkt(&self, dst_mac: &MacAddress, out_port: PortId) -> Option<NodeAction> {
        let frame = EthernetFrame::new(self.mac, dst_mac.clone(), 0x800, b"Hello".to_vec());
        let from  = (self.id, out_port);
        Some(NodeAction::Send {
            from,
            frame,
        })
    }

    fn rcv_pkt(&self, _frame: &EthernetFrame, _port: PortId) -> Option<NodeAction> {
        info!(
            node = %self.id,
            "Successfully received pkt at node"
        );
        None
    }
}
