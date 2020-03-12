// Copyright 2020 PolkaX

use crate::{Event, Planner, StateMachine};
use crossbeam::deque::Worker;
use std::thread;

pub struct StateThread {
    worker: Worker<Event>,
    join_handle: thread::JoinHandle<()>,
}

impl Planner for StateThread {
    fn plan(&self, events: &[Event]) {
        for event in events {
            self.worker.push(event.clone());
        }
    }
}

impl StateThread {
    pub fn run() -> Self {
        let worker = Worker::<Event>::new_fifo();
        let stealer = worker.stealer();

        let join_handle: thread::JoinHandle<_> = thread::spawn(|| {
            let mut state_machine = StateMachine::new(stealer);
            state_machine.run();
        });

        StateThread {
            worker,
            join_handle,
        }
    }
}
