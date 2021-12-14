pub mod turnloop {
    #[derive(PartialEq)]
    pub enum TurnLoopStatus {
        TurnRunning = 0,
        EndTurnWait = 1,
        WaitUntilCue = 2,
        Terminated = 3,
    }

    pub struct TurnLoopLifecycleManager {
        pub status: TurnLoopStatus,
        pub handler: Vec<dyn Fn(&Self) -> Result<(), Box<dyn std::error::Error>>>,
    }

    impl TurnLoopLifecycleManager {
        pub fn new() -> Self {
            TurnLoopLifecycleManager {
                status: TurnLoopStatus::WaitUntilCue,
                handler: Vec::new(),
            }
        }

        pub fn set_event_handler(
            &self,
            status: TurnLoopStatus,
            handler: dyn Fn(&Self) -> Result<(), Box<dyn std::error::Error>>,
        ) {
        }

        pub fn kickoff(&self) {
            while self.status != TurnLoopStatus::Terminated {
                let hdlr: dyn Fn(&Self) -> Result<(), Box<dyn std::error::Error>> =
                    self.handler[self.status as usize];
                hdlr();
            }
        }

        pub fn get_current_status(&self) -> TurnLoopStatus {
            self.status
        }

        pub fn on_turn_end(&self) -> () {
            while self.status == TurnLoopStatus::EndTurnWait {}
            ()
        }

        pub fn on_turn_resolution(&self) -> () {
            while self.status == TurnLoopStatus::WaitUntilCue {}
            ()
        }

        pub fn on_turn_start(&self) -> () {
            while self.status == TurnLoopStatus::TurnRunning {}
            ()
        }

        pub fn end_turn(&self) {
            self.status = TurnLoopStatus::EndTurnWait;
        }

        pub fn new_turn(&self) {
            self.status = TurnLoopStatus::WaitUntilCue;
        }

        pub fn start_turn(&self) {
            self.status = TurnLoopStatus::TurnRunning;
        }
    }
}

pub mod global {
    use std::sync::Arc;

    use futures::lock::Mutex;

    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub enum GlobalStatus {
        WaitUser = 0,
        WaitUserReady = 1,
        InitialServerLoading = 2,
        InitialClientLoadingWait = 3,
        GameRunning = 4,
        GameEndProcess = 5,
        WaitEndProcess = 6,
        RoomTerminated = 7,
    }

    pub struct GlobalLifecycleManager {
        pub status: GlobalStatus,
        pub event_handlers: Vec<Arc<Mutex<dyn FnOnce() -> Result<(), Box<dyn std::error::Error>>>>>,
    }

    impl GlobalLifecycleManager {
        pub fn new() -> Self {
            GlobalLifecycleManager {
                status: GlobalStatus::WaitUser,
                event_handlers: vec![],
            }
        }

        pub fn set_event_handler(
            &self,
            status: GlobalStatus,
            handler: Box<dyn FnOnce(&Self) -> Result<(), Box<dyn std::error::Error>>>,
        ) {
            let th_handler = Arc::new(Mutex::from(handler));
            self.event_handlers.insert(status as usize, th_handler);
        }

        pub fn kickoff(&mut self) {
            for hdlr in self.event_handlers {
                let handler: dyn FnOnce() -> Result<(), Box<dyn std::error::Error>> =
                    match hdlr.lock() {
                        Ok(v) => v,
                        Err(_) => panic!("Event stalled into deadlock!"),
                    };

                handler();
            }
        }

        pub fn get_current_status(&self) -> GlobalStatus {
            self.status
        }

        pub fn promote_status(&self, status: GlobalStatus) -> bool {
            // Counteracting is NOT allowed
            let recieved_status_number = status as u8;
            let current_status_number = self.status as u8;

            if recieved_status_number > current_status_number {
                self.status = status;
                return true;
            } else {
                return false;
            }
        }

        pub fn wait_until_status(&self, status: GlobalStatus) -> () {
            while (self.status as u8) >= (status as u8) {}
            ()
        }
    }
}
