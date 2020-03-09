// Copyright 2020 PolkaX

use crate::{EventError, EventRet, StateMachine};

pub trait Handler {
    fn handle(&self, state_machine: &mut StateMachine) -> Result<EventRet, EventError>;
}
