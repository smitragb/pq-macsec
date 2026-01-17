#![allow(dead_code)]

use crate::{nodes::Node, packet::EthernetFrame, simulator::SimTime};

#[derive(Debug, Clone, Copy)]
pub struct LinkConfig {
    pub end_a: Node,
    pub end_b: Node,
    pub delay: Option<u32>,
    pub corrupt_every: Option<u32>,
    pub drop_every: Option<u32>,
}

impl LinkConfig {
    pub fn new(end_a: Node, end_b: Node) -> Self {
        Self {
            end_a,
            end_b,
            delay: None,
            corrupt_every: None,
            drop_every: None,
        }
    }

    pub fn with_delay(mut self, delay: u32) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn with_corrupt(mut self, corrupt_every: u32) -> Self {
        self.corrupt_every = Some(corrupt_every);
        self
    }

    pub fn with_drop(mut self, drop_every: u32) -> Self {
        self.drop_every = Some(drop_every);
        self
    }
    
    pub fn swap_ends(&self) -> Self {
        Self {
            end_a: self.end_b,
            end_b: self.end_a,
            ..*self
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    pub fn handle_pkt(
        &mut self,
        pkt: EthernetFrame,
        current_time: SimTime,
    ) -> Option<(EthernetFrame, SimTime)> {
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
        Some((pkt, delivery_time))
    }
}
