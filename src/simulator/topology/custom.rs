use crate::{
    link::{Link, LinkBuilder, LinkEndId},
    log_frame,
    nodes::{Node, NodeAction, NodeBuilder, NodeHandler, NodeId},
    simulator::{event::Event, topology::Topology},
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{link::config::LinkConfig, nodes::builder::NodeConfig};

#[derive(Deserialize)]
pub struct TopologyConfig {
    pub nodes: Vec<NodeConfig>,
    pub links: Vec<LinkConfig>,
}

pub struct CustomTopology {
    nodes: HashMap<NodeId, Node>,
    links: HashMap<LinkEndId, Link>,
}

impl CustomTopology {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, node_cfg: Vec<NodeConfig>) -> Self {
        let nodes: Vec<Node> = node_cfg.into_iter().map(NodeBuilder::consume).collect(); 
        for node in nodes {
            self.nodes.insert(node.id(), node);
        }
        self
    }

    pub fn with_links(mut self, link_cfg: Vec<LinkConfig>) -> Self {
        let links: Vec<Link> = link_cfg.into_iter().map(LinkBuilder::consume).collect(); 
        for link in links {
            let (end_a, end_b) = link.get_ends();
            let rev_link = link.swap_ends();
            self.links.insert(end_a, link);
            self.links.insert(end_b, rev_link);
        }
        self
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn connect(self) -> Self {
        todo!()
    }
}

impl Topology for CustomTopology {
    fn handle_pkt(&mut self, event: Event) -> Option<Event> {
        let time = event.time;
        match event.action {
            NodeAction::Send { from, frame } => {
                let (id, port) = from;
                log_frame!("SEND", time, frame, port);
                let link = self
                    .links
                    .get_mut(&from)
                    .unwrap_or_else(|| unreachable!("Can't send out of node {id}"));

                let peer = link.get_peer(from);
                let (pkt_opt, del_time) = link.handle_pkt(frame.clone(), time);
                let pkt = pkt_opt?;
                Some(Event::new(
                    del_time,
                    NodeAction::Rcv {
                        to: peer,
                        frame: pkt,
                    },
                ))
            }
            NodeAction::Rcv { to, frame } => {
                let (id, port) = to;
                log_frame!("RECV", time, frame, port);
                let node = self
                    .nodes
                    .get(&id)
                    .unwrap_or_else(|| unreachable!("Can't find node {id} in chain"));
                let action = node.rcv_pkt(&frame, port)?;
                Some(Event::new(time, action))
            }
        }
    }
}
