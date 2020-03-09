// Copyright 2020 PolkaX

use crate::{Event, EventError, EventRet, EventType, Planner, StateMachine, StateThread};

#[test]
fn exit_test() {
    let state_thread = StateThread::run();
    let events = vec![Event::new(EventType::Exit)];
    state_thread.plan(&events);
}
