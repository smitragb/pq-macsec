use std::collections::{HashMap, HashSet};

use tracing::info;

use crate::{
    assert_or_log,
    link::PortId,
    nodes::{NodeAction, NodeHandler, NodeId},
    packet::{EthernetFrame, MacAddress},
    simulator::macros::format_mac,
};

pub struct SwitchingNode {
    pub id: NodeId,
    pub ports: HashSet<PortId>,
    pub mac: MacAddress,
    mac_address_table: HashMap<MacAddress, PortId>,
}

impl SwitchingNode {
    pub fn new(id: NodeId, ports: &Vec<PortId>, mac: &MacAddress) -> Self {
        Self {
            id,
            ports: ports.iter().cloned().collect(),
            mac: mac.clone(),
            mac_address_table: HashMap::new(),
        }
    }

    pub fn install_mac_entry(&mut self, mac: &MacAddress, port: PortId) {
        assert_or_log!(
            self.ports.contains(&port),
            node_id = %self.id,
            mac = %format_mac(mac),
            port = %port,
            "Node cannot install mac on port. It doesn't exist!",
        );
        self.mac_address_table.insert(*mac, port);
    }
}

impl NodeHandler for SwitchingNode {
    fn send_pkt(&self, _dst_mac: &MacAddress, _out_port: PortId) -> Option<NodeAction> {
        unreachable!("Cannot get here since switches are intermediate nodes")
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
                router = %self.id,
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
