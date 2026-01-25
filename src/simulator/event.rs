use std::collections::BTreeMap;

use crate::{
    link::PortId,
    nodes::{NodeAction, NodeHandler},
    packet::MacAddress,
    simulator::SimTime,
};

pub struct Event {
    pub time: SimTime,
    pub action: NodeAction,
}

impl Event {
    pub fn new(time: SimTime, action: NodeAction) -> Self {
        Self { time, action }
    }
}

pub struct EventHandler {
    pub time: SimTime,
    event_queue: BTreeMap<SimTime, Vec<Event>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            time: 0,
            event_queue: BTreeMap::new(),
        }
    }

    pub fn schedule(&mut self, event: Event) {
        self.event_queue
            .entry(event.time)
            .or_insert_with(Vec::new)
            .push(event);
    }

    pub fn schedule_send<T: NodeHandler>(
        &mut self,
        time: SimTime,
        n: &T,
        port: PortId,
        dst: &MacAddress,
    ) {
        if let Some(action) = n.send_pkt(dst, port) {
            self.schedule(Event::new(time, action));
        }
    }

    pub fn next_events(&mut self) -> Option<Vec<Event>> {
        let (&time, _) = self.event_queue.iter().next()?;
        self.time = time;
        self.event_queue.remove(&time)
    }
}
