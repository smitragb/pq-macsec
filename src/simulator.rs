#![allow(dead_code)]
use std::collections::{BTreeMap, HashMap};
use crate::{link::{Link, NodeId}, packet::EthernetFrame};
pub type SimTime = u64;

pub enum Event {
    SendPkt(EthernetFrame),
    RcvPkt(EthernetFrame),
}

pub struct Simulator {
    nodes: HashMap<NodeId, NodeId>,
    links: Vec<Link>,
    event_queue: BTreeMap<SimTime, Event>,
}
