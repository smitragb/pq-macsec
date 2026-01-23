use crate::{
    simulator::event::Event,
};
pub mod p2p;
pub mod chain;

pub trait Topology {
    fn handle_pkt(&mut self, event: Event) -> Option<Event>;
}

