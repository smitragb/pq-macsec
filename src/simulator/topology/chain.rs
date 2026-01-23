use std::collections::HashMap;

use crate::{
    assert_or_log,
    link::{Link, LinkConfig, LinkEndId, PortId},
    log_frame,
    nodes::{Node, NodeAction, NodeHandler, NodeId, bridging::ForwardingNode, simple::SimpleNode},
    packet::MacAddress,
    simulator::{event::Event, topology::Topology},
};

pub struct ChainTopology {
    pub num_nodes: u8,
    nodes: HashMap<NodeId, Node>,
    links: HashMap<LinkEndId, Link>,
}

impl ChainTopology {
    pub fn new(num_nodes: u8) -> Self {
        assert_or_log!(
            num_nodes >= 3,
            nodes = %num_nodes,
            "Cannot create a chain topology with nodes",
        );
        Self {
            num_nodes,
            nodes: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn build(mut self, macs: &Vec<MacAddress>, ports: &Vec<PortId>) -> Self {
        assert_or_log!(
            macs.len() == self.num_nodes as usize,
            "The number of MAC addresses doesn't match number of nodes"
        );
        assert_or_log!(
            ports.len() == 2 * (self.num_nodes as usize) - 2,
            "The number of ports doesn't match number of ports required for the chain",
        );
        self.create_nodes(macs, ports);
        self.update_mac_on_nodes(macs);
        self.create_links(ports);
        self
    }

    fn create_nodes(&mut self, macs: &[MacAddress], ports: &[PortId]) {
        let last = (self.num_nodes - 1) as usize;
        for (i, mac) in macs.iter().enumerate() {
            if i == 0 {
                let node = SimpleNode::new(i as u8, *mac, ports[0]);
                self.nodes.insert(i as u8, Node::Simple(node));
            } else if i == last {
                let node = SimpleNode::new(i as u8, *mac, ports[2*last - 1]);
                self.nodes.insert(i as u8, Node::Simple(node));
            } else {
                let left_port = ports[2*i - 1];
                let right_port = ports[2*i];
                let node = ForwardingNode::new(i as u8, [left_port, right_port], mac);
                self.nodes.insert(i as u8, Node::Forwarding(node));
            }
        }
    }

    fn update_mac_on_nodes(&mut self, macs: &[MacAddress]) {
        let last = (self.num_nodes - 1) as usize;
        for (mac_idx, mac) in macs.iter().enumerate() {
            for node_idx in 1..last {
                if mac_idx == node_idx {
                    continue;
                }

                let node = self.nodes.get_mut(&(node_idx as u8))
                    .unwrap_or_else(|| unreachable!("Node {node_idx} missing"));

                let fwd = match node {
                    Node::Forwarding(f) => f,
                    Node::Simple(_) => unreachable!("Cannot have SimpleNode here"),
                };

                let port = if mac_idx < node_idx {
                    fwd.left_port()
                } else {
                    fwd.right_port()
                };

                fwd.install_mac_entry(port, mac);
            }
        }
    }

    fn create_links(&mut self, ports: &[PortId]) {
        let mut insert_link = |(a_id, a_port), (b_id, b_port)| {
            let link = Link::new(LinkConfig::new(a_id, a_port, b_id, b_port));
            let rev_link = link.swap_ends();
            self.links.insert((a_id, a_port), link);
            self.links.insert((b_id, b_port), rev_link);
        };

        let last = (self.num_nodes - 1) as usize;
        for i in 0..last {
            insert_link((i as u8, ports[2 * i]), ((i + 1) as u8, ports[2 * i + 1]));
        }
    }
}

impl Topology for ChainTopology {
    fn handle_pkt(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::SendPkt { time, from, frame } => {
                let (id, port) = from;
                log_frame!("SEND", time, frame, port);
                let link = self.links.get_mut(&from)
                    .unwrap_or_else(|| unreachable!("Can't send out of node {id}"));

                let peer = link.get_peer(from);
                let (pkt_opt, del_time) = link.handle_pkt(frame.clone(), time);
                let pkt = pkt_opt?;

                Some(Event::RcvPkt {
                    time: del_time,
                    at: peer,
                    frame: pkt,
                })
            }
            Event::RcvPkt { time, at, frame } => {
                let (id, port) = at;
                log_frame!("RECV", time, frame, port);

                let node = self.nodes.get(&id)
                    .unwrap_or_else(|| unreachable!("Can't find node {id} in chain"));

                match node.rcv_pkt(&frame, port)? {
                    NodeAction::Send { from, frame } => {
                        Some(Event::SendPkt { 
                            time,
                            from,
                            frame,
                        })
                    },
                    _ => unreachable!(
                        "Can't get receive node action since we would return None instead"
                    ),
                }
            }
        }
    }
}
