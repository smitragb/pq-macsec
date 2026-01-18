use crate::{
    link::PortId,
    nodes::NodeHandler,
    packet::MacAddress,
    simulator::{event::EventHandler, topology::Topology},
};

pub mod event;
pub mod macros;
pub mod topology;
pub type SimTime = u64;

pub struct Simulator<T> {
    event_handler: EventHandler,
    topology: T,
}

impl<T: Topology> Simulator<T> {
    pub fn new(topology: T) -> Self {
        Self {
            event_handler: EventHandler::new(),
            topology,
        }
    }

    pub fn schedule_send<N: NodeHandler>(
        &mut self,
        time: SimTime,
        n: &N,
        port: PortId,
        dst: &MacAddress,
    ) {
        self.event_handler.schedule_send(time, n, port, dst);
    }

    pub fn run(&mut self) {
        while let Some(events) = self.event_handler.next_events() {
            for event in events {
                if let Some(next_event) = self.topology.handle_pkt(event) {
                    self.event_handler.schedule(next_event)
                }
            }
        }
    }
}
