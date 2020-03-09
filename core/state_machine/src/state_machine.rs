// Copyright 2020 PolkaX

use crate::event::SectorStart;
use crate::{Event, EventRet, Handler, SectorBuilder, SectorInfo, SectorState};
use crossbeam::deque::{Steal, Stealer};

pub struct StateMachine {
    state: SectorInfo,
    sb: SectorBuilder,
    stealer: Stealer<Event>,
}

impl StateMachine {
    pub fn new(stealer: Stealer<Event>) -> Self {
        StateMachine {
            state: SectorInfo::new(),
            sb: SectorBuilder {},
            stealer,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Steal::Success(event) = self.stealer.steal() {
                let ret = event.handle(self);
                if ret == Ok(EventRet::Exit) {
                    // To do: 退出清理操作
                    println!("recv Exit signal");
                    break;
                }
            }
            self.state_transition();
        }
    }

    pub fn handle_packing(&self, sector_start: &SectorStart) {}

    /////
    // Now decide what to do next

    /*

        *   Empty
        |   |
        |   v
        *<- Packing <- incoming
        |   |
        |   v
        *<- Unsealed <--> SealFailed
        |   |
        |   v
        *   PreCommitting <--> PreCommitFailed
        |   |                  ^
        |   v                  |
        *<- WaitSeed ----------/
        |   |||
        |   vvv      v--> SealCommitFailed
        *<- Committing
        |   |        ^--> CommitFailed
        |   v             ^
        *<- CommitWait ---/
        |   |
        |   v
        *<- Proving
        |
        v
        FailedUnrecoverable

        UndefinedSectorState <- ¯\_(ツ)_/¯
            |                     ^
            *---------------------/

    */
    fn state_transition(&mut self) {
        match self.state.state {
            SectorState::Empty => {}
            SectorState::Packing => {}
            SectorState::Unsealed => {}
            SectorState::PreCommitting => {}
            SectorState::WaitSeed => {}
            SectorState::Committing => {}
            SectorState::CommitWait => {}
            SectorState::Proving => {}
            SectorState::FailedUnrecoverable => {}
            SectorState::UndefinedSectorState => {}
            _ => {}
        }
    }
}
