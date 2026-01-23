use crate::{
    link::{Link, LinkConfig, PortId},
    log_frame,
    nodes::{NodeHandler, NodeId, simple::SimpleNode},
    simulator::{event::Event, topology::Topology},
};

pub struct P2PConnection {
    node_a: SimpleNode,
    node_b: SimpleNode,
    a_to_b: Link,
    b_to_a: Link,
}

impl P2PConnection {
    pub fn with_nodes(node_a: &SimpleNode, node_b: &SimpleNode) -> Self {
        let link = Link::new(LinkConfig::new(
            node_a.id,
            node_a.port,
            node_b.id,
            node_b.port,
        ));
        let rev_link = link.swap_ends();
        Self {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
            a_to_b: link,
            b_to_a: rev_link,
        }
    }

    pub fn full_specification(node_a: &SimpleNode, node_b: &SimpleNode, link: &Link) -> Self {
        let rev_link = link.swap_ends();
        Self {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
            a_to_b: link.clone(),
            b_to_a: rev_link,
        }
    }

    fn get_peer_from_id(&self, id: NodeId, port: PortId) -> SimpleNode {
        let id_match = self.node_a.id == id;
        let port_match = self.node_a.port == port;
        match (id_match, port_match) {
            (true, true) => self.node_b.clone(),
            (false, false) => self.node_a.clone(),
            (_, _) => unreachable!("ID and Port don't match with a single node"),
        }
    }

    fn get_node_from_id(&self, id: NodeId, port: PortId) -> SimpleNode {
        let id_match = self.node_a.id == id;
        let port_match = self.node_a.port == port;
        match (id_match, port_match) {
            (true, true) => self.node_a.clone(),
            (false, false) => self.node_b.clone(),
            (_, _) => unreachable!("ID and Port don't match with a single node"),
        }
    }

    fn get_link_from_id(&self, id: NodeId, port: PortId) -> Link {
        let id_match = self.node_a.id == id;
        let port_match = self.node_a.port == port;
        match (id_match, port_match) {
            (true, true) => self.a_to_b.clone(),
            (false, false) => self.b_to_a.clone(),
            (_, _) => unreachable!("ID and Port don't match with a single node"),
        }
    }
}

impl Topology for P2PConnection {
    fn handle_pkt(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::SendPkt {
                time,
                from,
                frame,
            } => {
                let (id, port) = from;
                log_frame!("SEND", time, frame, port);
                let peer = self.get_peer_from_id(id, port);
                let at = (peer.id, peer.port); 
                let mut link = self.get_link_from_id(id, port);
                if let (Some(pkt), del_time) = link.handle_pkt(frame.clone(), time) {
                    Some(Event::RcvPkt {
                        time: del_time,
                        at,
                        frame: pkt,
                    })
                } else {
                    None
                }
            }
            Event::RcvPkt {
                time,
                at,
                frame,
            } => {
                let (id, port) = at;
                log_frame!("RECV", time, frame, port);
                let node = self.get_node_from_id(id, port);
                node.rcv_pkt(&frame, port);
                None
            }
        }
    }
}
