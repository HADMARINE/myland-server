#[derive(PartialEq)]
pub enum MainLoopStatus {
    TurnRunning,
    EndTurnWait,
    WaitUntilCue,
}

struct MainLoop {
    pub loop_status: MainLoopStatus,
}

impl MainLoop {
    pub fn new() -> Self {
        MainLoop {
            loop_status: MainLoopStatus::WaitUntilCue,
        }
    }

    pub fn on_turn_end(&self) -> () {
        while self.loop_status == MainLoopStatus::EndTurnWait {}
        ()
    }

    pub fn on_turn_resolution(&self) -> () {
        while self.loop_status == MainLoopStatus::WaitUntilCue {}
        ()
    }

    pub fn on_turn_start(&self) -> () {
        while self.loop_status == MainLoopStatus::TurnRunning {}
        ()
    }

    pub fn end_turn(&self) {
        self.loop_status = MainLoopStatus::EndTurnWait;
    }

    pub fn new_turn(&self) {
        self.loop_status = MainLoopStatus::WaitUntilCue;
    }

    pub fn start_turn(&self) {
        self.loop_status = MainLoopStatus::TurnRunning;
    }
}
