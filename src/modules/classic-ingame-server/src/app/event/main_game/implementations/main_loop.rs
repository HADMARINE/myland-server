use crate::app::event::main_game::lifecycle_manager::turnloop::{
    TurnLoopLifecycleManager, TurnLoopStatus,
};

pub fn main_loop() {
    let tlm = TurnLoopLifecycleManager::new();
    tlm.set_event_handler(TurnLoopStatus::TurnRunning, turn_running);
    tlm.set_event_handler(TurnLoopStatus::EndTurnWait, end_turn_wait);
    tlm.set_event_handler(TurnLoopStatus::WaitUntilCue, wait_until_cue);

    tlm.tlm.kickoff();
}

fn turn_running() {}

fn end_turn_wait() {}

fn wait_until_cue() {}
