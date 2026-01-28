use serde::Deserialize;

use crate::{
    link::{LinkEndId, PortId},
    nodes::NodeId,
};

#[derive(Debug, Clone, Deserialize)]
pub struct LinkConfig {
    pub end_a: LinkEndId,
    pub end_b: LinkEndId,
    pub delay: Option<u32>,
    pub corrupt_every: Option<u32>,
    pub drop_every: Option<u32>,
}

impl LinkConfig {
    pub fn new(a_id: NodeId, a_port: PortId, b_id: NodeId, b_port: PortId) -> Self {
        let end_a = (a_id, a_port);
        let end_b = (b_id, b_port);
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
