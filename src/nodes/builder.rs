use serde::Deserialize;

use crate::{link::PortId, nodes::NodeId, packet::MacAddress};

#[derive(Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum NodeConfig {
    #[serde(rename = "simple")]
    Simple {
        id: NodeId,
        port: PortId,
        mac: MacAddress,
    },

    #[serde(rename = "forwarding")]
    Forwarding {
        id: NodeId,
        ports: [PortId; 2],
        mac: MacAddress,
    },

    #[serde(rename = "switch")]
    Switch {
        id: NodeId,
        ports: Vec<PortId>,
        mac: MacAddress,
    },
}
