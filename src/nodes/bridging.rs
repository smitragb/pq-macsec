use std::collections::HashMap;

use tracing::info;

use crate::{
    assert_or_log,
    link::{LinkEndId, PortId},
    nodes::{NodeAction, NodeHandler, NodeId},
    packet::{EthernetFrame, MacAddress},
    simulator::macros::format_mac,
};

pub struct ForwardingNode {
    pub id: NodeId,
    pub ports: [PortId; 2], 
    pub mac: MacAddress,
    mac_address_table: HashMap<MacAddress, PortId>,
}

impl ForwardingNode {
    pub fn new(id: NodeId, ports: &[PortId; 2], mac: &MacAddress) -> Self {
        Self {
            id,
            ports: *ports,
            mac: *mac,
            mac_address_table: HashMap::new(),
        }
    }

    pub fn install_mac_entry(&mut self, port: PortId, mac: &MacAddress) {
        assert_or_log!(
            self.ports.contains(&port),
            node_id = %self.id,
            mac = %format_mac(mac),
            port = %port,
            "Node cannot install mac on port. It doesn't exist!",
        );
        self.mac_address_table.insert(*mac, port);
    }

    pub fn left_port(&self) -> PortId {
        self.ports[0]
    }

    pub fn right_port(&self) -> PortId {
        self.ports[1]
    }

    pub fn left_link_id(&self) -> LinkEndId {
        (self.id, self.ports[0])
    }

    pub fn right_link_id(&self) -> LinkEndId {
        (self.id, self.ports[0])
    }
}

impl NodeHandler for ForwardingNode {
    fn send_pkt(&self, dst_mac: &MacAddress, out_port: PortId) -> Option<NodeAction> {
        assert_or_log!(
            self.ports.contains(&out_port),
            node_id = %self.id,
            port = %out_port,
            "Node cannot send pkts out of port. It doesn't exist",
        );
        let frame = EthernetFrame::new(self.mac, dst_mac.clone(), 0x800, b"Hello".to_vec());
        let from  = (self.id, out_port);
        Some(NodeAction::Send {
            from,
            frame,
        })
    }

    fn rcv_pkt(&self, frame: &EthernetFrame, port: PortId) -> Option<NodeAction> {
        let src_mac = frame.src_mac;
        assert_or_log!(
            self.mac_address_table.get(&src_mac) == Some(&port),
            node = %self.id,
            src = %format_mac(&src_mac),
            port = %port,
            "Node received pkt from unknown source on port. \
            Not supporting dynamic MAC address learning",
        );

        let dst_mac = frame.dst_mac;
        if self.mac == dst_mac {
            info!(
                node = %self.id,
                "Successfully received pkt at"
            );
            return None;
        }

        let out_port = self
            .mac_address_table
            .get(&dst_mac)
            .expect("Not supporting dynamic MAC address learning");
        let from = (self.id, *out_port);    

        Some(NodeAction::Send {
            from,
            frame: frame.clone(),
        })
    }
}
