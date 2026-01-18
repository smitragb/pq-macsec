use crate::{
    link::{Link, LinkConfig, PortId},
    nodes::{Node, NodeAction, NodeId},
    packet::{EthernetFrame, MacAddress},
};
use std::collections::{BTreeMap, HashMap};
pub type SimTime = u64;

#[derive(Debug)]
pub enum Event {
    SendPkt {
        from: NodeId,
        port: PortId,
        frame: EthernetFrame,
    },
    RcvPkt {
        at: NodeId,
        port: PortId,
        frame: EthernetFrame,
    },
}

fn format_mac(mac: &MacAddress) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}
macro_rules! log_frame {
    ($action:expr, $time:expr, $frame:expr, $port:expr) => {
        let src_mac = format_mac(&$frame.src_mac);
        let dst_mac = format_mac(&$frame.dst_mac);
        let ethertype = format!("0x{:04x}", $frame.ethertype);
        match $action {
            "SEND" => {
                tracing::info!(
                    time = $time, outgoing_port = $port, src_mac = src_mac,
                    dst_mac = dst_mac, ethertype = ethertype,
                    payload = String::from_utf8_lossy(&$frame.payload).as_ref(),
                    $action,
                );
            },
            "RECV" => {
                tracing::info!(
                    time = $time, incoming_port = $port, src_mac = src_mac,
                    dst_mac = dst_mac, ethertype = ethertype,
                    payload = String::from_utf8_lossy(&$frame.payload).as_ref(),
                    $action,
                );
            },
            _      => {
                tracing::info!(
                    time = $time, src_mac = src_mac, dst_mac = dst_mac, 
                    ethertype = ethertype,
                    payload = String::from_utf8_lossy(&$frame.payload).as_ref(),
                    $action,
                );
            },
        }
    };
}

pub struct Simulator {
    time: SimTime,
    nodes: HashMap<NodeId, Node>,
    links: HashMap<(NodeId, PortId), Link>,
    event_queue: BTreeMap<SimTime, Vec<Event>>,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            time: 0,
            nodes: HashMap::new(),
            links: HashMap::new(),
            event_queue: BTreeMap::new(),
        }
    }

    pub fn with_nodes(mut self, node_list: Vec<Node>) -> Self {
        self.nodes = node_list.into_iter().map(|a| (a.id, a)).collect();
        self
    }

    pub fn add_node(&mut self, node: &Node) {
        self.nodes.insert(node.id, node.clone());
    }

    pub fn with_links(mut self, links: Vec<Link>) -> Self {
        for link in links {
            let a = link.config.end_a;
            let port_a = link.config.port_a;
            let b = link.config.end_b;
            let port_b = link.config.port_b;
            let rev_link = link.swap_ends();
            self.links.insert((a, port_a), link);
            self.links.insert((b, port_b), rev_link);
        }
        self
    }

    pub fn connect(&mut self, a: NodeId, port_a: PortId, b: NodeId, port_b: PortId) {
        let link = Link::new(LinkConfig::new(a, port_a, b, port_b));
        let rev_link = Link::new(LinkConfig::new(b, port_b, a, port_a));
        self.nodes.get_mut(&a).unwrap().update_port(port_a, &link);
        self.nodes
            .get_mut(&b)
            .unwrap()
            .update_port(port_b, &rev_link);
        self.links.insert((a, port_a), link);
        self.links.insert((b, port_b), rev_link);
    }

    pub fn schedule_send(
        &mut self,
        time: SimTime,
        n: &Node,
        port: PortId,
        dst: &MacAddress,
        payload: Vec<u8>,
    ) {
        if let Some(action) = n.send_pkt(port, dst, payload) {
            if let NodeAction::Send { from, port, frame } = action {
                self.schedule(time, Event::SendPkt { from, port, frame });
            }
        }
    }

    fn schedule(&mut self, time: SimTime, event: Event) {
        self.event_queue
            .entry(time)
            .or_insert_with(Vec::new)
            .push(event);
    }

    fn process_events(&mut self, time: SimTime, events: &Vec<Event>) {
        for event in events {
            match event {
                Event::SendPkt { from, port, frame } => {
                    log_frame!("SEND", time, frame, port);
                    if let Some(peer) = self.links.get_mut(&(*from, *port)) {
                        let peer_id = peer.config.end_b;
                        let peer_port = peer.config.port_b;
                        if let (Some(pkt), del_time) = peer.handle_pkt(frame.clone(), time) {
                            self.schedule(
                                del_time,
                                Event::RcvPkt {
                                    at: peer_id,
                                    port: peer_port,
                                    frame: pkt,
                                },
                            );
                        }
                    }
                }
                Event::RcvPkt { at, port, frame } => {
                    if let Some(at_node) = self.nodes.get(at) {
                        log_frame!("RECV", time, frame, port);
                        at_node.rcv_pkt(frame, *port);
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        while let Some((&time, _)) = self.event_queue.iter().next() {
            self.time = time;
            let events = self.event_queue.remove(&time).unwrap();
            self.process_events(time, &events);
        }
    }
}
