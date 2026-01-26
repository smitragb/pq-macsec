use crate::{
    link::{LinkEndId, PortId},
    nodes::{bridging::ForwardingNode, simple::SimpleNode, switch::SwitchingNode},
    packet::{EthernetFrame, MacAddress},
};
pub mod bridging;
pub mod simple;
pub mod switch;

pub type NodeId = u8;
pub trait NodeHandler {
    fn send_pkt(&self, dst_mac: &MacAddress, out_port: PortId) -> Option<NodeAction>;
    fn rcv_pkt(&self, frame: &EthernetFrame, port: PortId) -> Option<NodeAction>;
}

pub enum NodeAction {
    Send {
        from: LinkEndId,
        frame: EthernetFrame,
    },
    Rcv {
        to: LinkEndId,
        frame: EthernetFrame,
    },
}

pub enum Node {
    Simple(SimpleNode),
    Forwarding(ForwardingNode),
    Switch(SwitchingNode),
}

impl NodeHandler for Node {
    fn send_pkt(&self, dst_mac: &MacAddress, out_port: PortId) -> Option<NodeAction> {
        match self {
            Node::Simple(s) => s.send_pkt(dst_mac, out_port),
            Node::Forwarding(f) => f.send_pkt(dst_mac, out_port),
            Node::Switch(sw) => sw.send_pkt(dst_mac, out_port),
        }
    }

    fn rcv_pkt(&self, frame: &EthernetFrame, port: PortId) -> Option<NodeAction> {
        match self {
            Node::Simple(s) => s.rcv_pkt(frame, port),
            Node::Forwarding(f) => f.rcv_pkt(frame, port),
            Node::Switch(sw) => sw.rcv_pkt(frame, port),
        }
    }
}

impl Node {
    pub fn id(&self) -> NodeId {
        match self {
            Node::Simple(s) => s.id, 
            Node::Forwarding(f) => f.id,
            Node::Switch(sw) => sw.id,
        }
    }

    pub fn mac(&self) -> MacAddress {
        match self {
            Node::Simple(s) => s.mac,
            Node::Forwarding(f) => f.mac,
            Node::Switch(sw) => sw.mac,
        }

    }
}

#[macro_export]
macro_rules! assert_or_log {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            tracing::error!($($arg)*);
            panic!("Assertion failed: {}", stringify!($cond));
        }
    };
}
