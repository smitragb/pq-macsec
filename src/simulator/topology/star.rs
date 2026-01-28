use std::collections::HashMap;

use crate::{
    assert_or_log,
    link::{Link, config::LinkConfig, LinkEndId, PortId},
    log_frame,
    nodes::{Node, NodeAction, NodeHandler, NodeId, simple::SimpleNode, switch::SwitchingNode},
    packet::MacAddress,
    simulator::{event::Event, topology::Topology},
};

pub struct StarTopology {
    pub num_nodes: u8,
    nodes: HashMap<NodeId, Node>,
    links: HashMap<LinkEndId, Link>,
    switch_id: NodeId,
    switch_ports: Vec<PortId>,
}

impl StarTopology {
    pub fn new(num_nodes: u8) -> Self {
        assert_or_log!(
            num_nodes >= 2,
            "Number of nodes must be at least 2 for start topology"
        );
        Self {
            num_nodes,
            nodes: HashMap::new(),
            links: HashMap::new(),
            switch_id: 0,
            switch_ports: Vec::new(),
        }
    }

    pub fn with_switch(mut self, n: SwitchingNode) -> Self {
        let id = n.id;
        let ports = n.ports.iter().cloned().collect();
        self.nodes.insert(id, Node::Switch(n));
        self.switch_id = id;
        self.switch_ports = ports;
        self
    }

    pub fn with_nodes(mut self, nodes: Vec<Node>) -> Self {
        assert_or_log!(
            nodes.len() == (self.num_nodes - 1) as usize,
            "Cannot have more nodes than specified in new()"
        );
        for node in nodes {
            let id = node.id();
            self.nodes.insert(id, node);
        }
        self
    }

    pub fn with_links(mut self, links: Vec<Link>) -> Self {
        assert_or_log!(
            links.len() == (self.num_nodes - 1) as usize,
            "Cannot have more links than specified in new()"
        );
        for link in links {
            let (end_a, end_b) = link.get_ends();
            let rev_link = link.swap_ends();
            self.links.insert(end_a, link);
            self.links.insert(end_b, rev_link);
        }
        self
    }

    pub fn add_switch(mut self, mac: &MacAddress, ports: &Vec<PortId>) -> Self {
        assert_or_log!(
            ports.len() == (self.num_nodes - 1) as usize,
            "Cannot have more ports than number of nodes in the network",
        );
        let n = SwitchingNode::new(0, ports, mac);
        self.nodes.insert(0, Node::Switch(n));
        self.switch_ports = ports.clone();
        self
    }

    pub fn add_nodes(mut self, macs: &Vec<MacAddress>) -> Self {
        assert_or_log!(
            macs.len() == (self.num_nodes - 1) as usize,
            "Cannot have more than {} including the switch",
            self.num_nodes
        );
        for (i, &mac) in macs.iter().enumerate() {
            let id = (i + 1) as u8;
            let n = SimpleNode::new(id, &mac, 0);
            self.nodes.insert(id, Node::Simple(n));
        }
        self.update_switch(macs);
        self
    }

    pub fn add_links(mut self) -> Self {
        for (i, &port) in self.switch_ports.iter().enumerate() {
            let node_id = (i + 1) as u8;
            let link = Link::new(LinkConfig::new(self.switch_id, port, node_id, 0));
            let rev_link = link.swap_ends();
            self.links.insert((self.switch_id, port), link);
            self.links.insert((node_id, 0), rev_link);
        }
        self
    }

    fn update_switch(&mut self, macs: &Vec<MacAddress>) {
        let mapping: Vec<(MacAddress, PortId)> = macs
            .iter()
            .cloned()
            .zip(self.switch_ports.iter().copied())
            .collect();

        let switch = self
            .nodes
            .get_mut(&self.switch_id)
            .expect("Create SwitchingNode first");
        let Node::Switch(s) = switch else {
            unreachable!("Need the node at switch_id to be a SwitchingNode");
        };
        for (mac, port) in mapping {
            s.install_mac_entry(&mac, port);
        }
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id) 
    } 
}

impl Topology for StarTopology {
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
