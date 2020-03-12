// Copyright 2020 PolkaX

use crate::{Handler, Piece, SectorInfo, StateMachine};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventRet {
    Exit,
    OK,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectorStart {
    id: u64,
    pieces: Vec<Piece>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Exit,
    Packing(SectorStart),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    event_type: EventType,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Event { event_type }
    }
}

impl Handler for Event {
    fn handle(&self, state_machine: &mut StateMachine) -> Result<EventRet, EventError> {
        match &self.event_type {
            EventType::Exit => Ok(EventRet::Exit),
            EventType::Packing(sector_start) => {
                state_machine.handle_packing(sector_start);
                Ok(EventRet::OK)
            }
        }
    }
}
