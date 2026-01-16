#![allow(dead_code)]
use crate::{
    link::{Link, LinkConfig},
    nodes::{Node, NodeId},
    packet::EthernetFrame,
};
use std::collections::{BTreeMap, HashMap};
pub type SimTime = u64;

pub enum Event {
    SendPkt { from: Node, frame: EthernetFrame },
    RcvPkt { at: Node, frame: EthernetFrame },
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

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn with_links(mut self, links: Vec<Link>) -> Self {
        for link in links {
            let a = link.config.end_a;
            let b = link.config.end_b;
            let rev_link = Link::new(LinkConfig::new(b, a));
            self.links.insert(a.id, link);
            self.links.insert(b.id, rev_link);
        }
        self
    }

    pub fn connect(&mut self, a: Node, b: Node) {
        let link = Link::new(LinkConfig::new(a, b));
        let rev_link = Link::new(LinkConfig::new(b, a));
        self.links.insert(a.id, link);
        self.links.insert(b.id, rev_link);
    }

    pub fn schedule(&mut self, time: SimTime, event: Event) {
        self.event_queue
            .entry(time)
            .or_insert_with(Vec::new)
            .push(event);
    }

    fn process_events(&mut self, time: SimTime, events: &Vec<Event>) {
        for event in events {
            match event {
                Event::SendPkt { from, frame } => {
                    if let Some(peer) = self.links.get(&from.id) {
                        let deliver_time = if let Some(delay) = peer.config.delay {
                            time + delay as u64
                        } else {
                            time
                        };
                        self.schedule(
                            deliver_time,
                            Event::RcvPkt {
                                at: peer.config.end_b,
                                frame: frame.clone(),
                            },
                        );
                    }
                }
                Event::RcvPkt { at, frame } => {
                    let _ = at.rcv_pkt(frame);
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
