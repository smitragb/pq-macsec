#![allow(dead_code)]

use crate::{nodes::NodeId, packet::EthernetFrame, simulator::SimTime};
pub type PortId = u8;

#[derive(Debug, Clone)]
pub struct LinkConfig {
    pub end_a: NodeId,
    pub port_a: PortId,
    pub end_b: NodeId,
    pub port_b: PortId,
    pub delay: Option<u32>,
    pub corrupt_every: Option<u32>,
    pub drop_every: Option<u32>,
}

impl LinkConfig {
    pub fn new(end_a: NodeId, port_a: PortId, end_b: NodeId, port_b: PortId) -> Self {
        Self {
            end_a,
            port_a,
            end_b,
            port_b,
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
            port_a: self.port_b,
            end_b: self.end_a,
            port_b: self.port_a,
            ..*self
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn handle_pkt (
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
