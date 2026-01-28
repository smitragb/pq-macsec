use crate::{link::config::LinkConfig, nodes::NodeId, packet::EthernetFrame, simulator::SimTime};
pub mod config;
pub type PortId = u8;

pub type LinkEndId = (NodeId, PortId);

#[derive(Clone)]
pub struct Link {
    pub config: LinkConfig,
    packet_count: u32,
}

impl Link {
    pub fn new(config: LinkConfig) -> Self {
        Self {
            config,
            packet_count: 0,
        }
    }

    pub fn swap_ends(&self) -> Self {
        Self {
            config: self.config.swap_ends(),
            ..*self
        }
    }

    pub fn get_peer(&self, end: LinkEndId) -> LinkEndId {
        if self.config.end_a == end {
            self.config.end_b
        } else {
            self.config.end_a
        }
    }

    pub fn get_ends(&self) -> (LinkEndId, LinkEndId) {
        (self.config.end_a, self.config.end_b)
    }

    pub fn handle_pkt(
        &mut self,
        pkt: EthernetFrame,
        current_time: SimTime,
    ) -> (Option<EthernetFrame>, SimTime) {
        self.packet_count = self.packet_count.wrapping_add(1);

        if let Some(corrupt_every) = self.config.corrupt_every {
            if corrupt_every > 0 && self.packet_count % corrupt_every == 0 {
                todo!("Need to add corruption logic");
            }
        }

        if let Some(drop_every) = self.config.drop_every {
            if drop_every > 0 && self.packet_count % drop_every == 0 {
                todo!("Need to add drop logic");
            }
        }

        let delivery_time = if let Some(delay) = self.config.delay {
            current_time.wrapping_add(delay as u64)
        } else {
            current_time
        };
        (Some(pkt), delivery_time)
    }
}

pub struct LinkBuilder;

impl LinkBuilder {
    pub fn consume(cfg: LinkConfig) -> Link {
        Link::new(cfg) 
    }

    pub fn build(cfg: &LinkConfig) -> Link {
        Link::new(cfg.clone())
    }
}

