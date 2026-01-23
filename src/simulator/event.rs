use std::collections::BTreeMap;

use crate::{
    link::{LinkEndId, PortId},
    nodes::{NodeAction, NodeHandler},
    packet::{EthernetFrame, MacAddress},
    simulator::SimTime,
};

#[derive(Debug)]
pub enum Event {
    SendPkt {
        time: SimTime,
        from: LinkEndId,
        frame: EthernetFrame,
    },
    RcvPkt {
        time: SimTime,
        at: LinkEndId,
        frame: EthernetFrame,
    },
}

impl Event {
    pub fn time(&self) -> SimTime {
        match self {
            Event::SendPkt { time, .. } => *time,
            Event::RcvPkt { time, .. } => *time,
        }
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
        let time = event.time();
        self.event_queue
            .entry(time)
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
            if let NodeAction::Send { from, frame } = action {
                self.schedule(Event::SendPkt {
                    time,
                    from,
                    frame,
                });
            }
        }
    }

    pub fn next_events(&mut self) -> Option<Vec<Event>> {
        if let Some((&time, _)) = self.event_queue.iter().next() {
            self.time = time;
            self.event_queue.remove(&time)
        } else {
            None
        }
    }
}
