// Copyright 2020 PolkaX

mod event;
mod handler;
mod sector_info;
mod state;
mod state_machine;
#[cfg(test)]
mod test;
mod thread;

pub use event::{Event, EventError, EventRet, EventType};
pub use handler::Handler;
pub use sector_info::{Piece, SectorInfo};
pub use state::SectorState;
use state_machine::StateMachine;
pub use thread::StateThread;

pub struct SectorBuilder {}

pub trait Planner {
    fn plan(&self, events: &[Event]);
}
