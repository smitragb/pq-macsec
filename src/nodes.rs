use crate::{
    link::{LinkEndId, PortId}, nodes::{bridging::ForwardingNode, simple::SimpleNode}, packet::{EthernetFrame, MacAddress}
};
pub mod bridging;
pub mod simple;

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
}

impl NodeHandler for Node {
    fn send_pkt(&self, dst_mac: &MacAddress, out_port: PortId) -> Option<NodeAction> {
        match self {
            Node::Simple(s) => s.send_pkt(dst_mac, out_port),
            Node::Forwarding(f) => f.send_pkt(dst_mac, out_port),
        }
    }

    fn rcv_pkt(&self, frame: &EthernetFrame, port: PortId) -> Option<NodeAction> {
        match self {
            Node::Simple(s) => s.rcv_pkt(frame, port),
            Node::Forwarding(f) => f.rcv_pkt(frame, port),
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
