use std::time::Instant;

use crate::app::event::main_game::{lifecycle_manager::turnloop::{
    TurnLoopLifecycleManager, TurnLoopStatus,
}, constants::TIME_PER_TURN_SEC};

pub fn main_loop() {
    let tlm = TurnLoopLifecycleManager::new();
    tlm.set_event_handler(TurnLoopStatus::TurnRunning, turn_running);
    tlm.set_event_handler(TurnLoopStatus::EndTurnWait, end_turn_wait);
    tlm.set_event_handler(TurnLoopStatus::WaitUntilCue, wait_until_cue);

    tlm.kickoff();
}

fn turn_running(tlm: &TurnLoopLifecycleManager) {
    let end_turn_ctrl = move || {
        let start_time = Instant::now();
        while start_time.elapsed().as_secs() < TIME_PER_TURN_SEC {}
        tlm.end_turn();
    }
}

fn end_turn_wait(tlm: &TurnLoopLifecycleManager) {
    tlm.new_turn();
}

fn wait_until_cue(tlm: &TurnLoopLifecycleManager) {}
