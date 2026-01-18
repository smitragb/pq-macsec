use crate::{
    link::{Link, LinkConfig},
    nodes::{Node, NodeAction, NodeId},
    packet::{EthernetFrame, MacAddress},
};
use std::collections::{BTreeMap, HashMap};
pub type SimTime = u64;

#[derive(Debug)]
pub enum Event {
    SendPkt { from: NodeId, frame: EthernetFrame },
    RcvPkt { at: NodeId, frame: EthernetFrame },
}

macro_rules! log_frame {
    ($action:expr, $time:expr, $frame:expr) => {
        tracing::info!(
            time = $time,
            src_mac = format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                        $frame.src_mac[0], $frame.src_mac[1], $frame.src_mac[2],
                        $frame.src_mac[3], $frame.src_mac[4], $frame.src_mac[5]),
            dst_mac = format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                        $frame.dst_mac[0], $frame.dst_mac[1], $frame.dst_mac[2],
                        $frame.dst_mac[3], $frame.dst_mac[4], $frame.dst_mac[5]),
            ethertype = format!("0x{:04x}", $frame.ethertype),
            payload = String::from_utf8_lossy(&$frame.payload).as_ref(),
            $action
        );
    };
}

pub struct Simulator {
    time: SimTime,
    nodes: HashMap<NodeId, Node>,
    links: HashMap<NodeId, Link>,
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
            let b = link.config.end_b;
            let rev_link = link.swap_ends();
            self.links.insert(a, link);
            self.links.insert(b, rev_link);
        }
        self
    }

    pub fn connect(&mut self, a: &Node, b: &Node) {
        let link = Link::new(LinkConfig::new(a, b));
        let rev_link = Link::new(LinkConfig::new(b, a));
        self.links.insert(a.id, link);
        self.links.insert(b.id, rev_link);
    }

    pub fn schedule_send(&mut self, time: SimTime, n: &Node, dst: &MacAddress, payload: Vec<u8>) {
        if let Some(action) = n.send_pkt(dst, payload) {
            if let NodeAction::Send { from, frame } = action {
                self.schedule(time, Event::SendPkt { from, frame });
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
                Event::SendPkt { from, frame } => {
                    log_frame!("SEND", time, frame);
                    if let Some(peer) = self.links.get_mut(&from) {
                        let peer_id = peer.config.end_b;
                        if let (Some(pkt), del_time) = peer.handle_pkt(frame.clone(), time) {
                            self.schedule(
                                del_time,
                                Event::RcvPkt { at: peer_id, frame: pkt }
                            );
                        }
                    }
                }
                Event::RcvPkt { at, frame } => {
                    if let Some(at_node) = self.nodes.get(at) {
                        log_frame!("RECV", time, frame);
                        at_node.rcv_pkt(frame);
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
