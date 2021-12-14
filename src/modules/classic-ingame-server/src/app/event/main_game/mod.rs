use std::{collections::HashMap, process::exit, time::Instant};

use self::{
    constants::USER_CONNECT_WAIT_TIME_SEC,
    lifecycle_manager::global::{GlobalLifecycleManager, GlobalStatus},
};

use super::EventMapType;

pub mod constants;
pub mod cyclic_event_queue;
pub mod game_manager;
pub mod implementations;
pub mod lifecycle_manager;
pub mod structs;
pub mod transformers;

pub fn get_config() -> EventMapType {
    let mut map: EventMapType = HashMap::new();

    map
}

pub fn start_game() {
    let glm = GlobalLifecycleManager::new();
    glm.set_event_handler(GlobalStatus::WaitUser, on_wait_user);
    glm.set_event_handler(GlobalStatus::WaitUserReady, on_wait_user_ready);
    glm.set_event_handler(
        GlobalStatus::InitialServerLoading,
        on_initial_server_loading,
    );
    glm.set_event_handler(
        GlobalStatus::InitialClientLoadingWait,
        on_initial_client_loading_wait,
    );
    glm.set_event_handler(GlobalStatus::GameRunning, on_game_running);
    glm.set_event_handler(GlobalStatus::GameEndProcess, on_game_end_process);
    glm.set_event_handler(GlobalStatus::WaitEndProcess, on_wait_end_process);
    glm.set_event_handler(GlobalStatus::RoomTerminated, on_room_terminated);
}

fn on_wait_user(glm: &GlobalLifecycleManager) {
    let start_time = Instant::now();
    while game_manager::waiting_users.len() == 0 {
        let elapsed = start_time.elapsed().as_secs();
        if elapsed > USER_CONNECT_WAIT_TIME_SEC {
            break;
        }
    }
    glm.promote_status(GlobalStatus::WaitUserReady);
}

fn on_wait_user_ready(glm: &GlobalLifecycleManager) {
    while game_manager::registered_users.len() == game_manager::ready_count {}
    glm.promote_status(GlobalStatus::InitialServerLoading);
}

fn on_initial_server_loading(glm: &GlobalLifecycleManager) {
    glm.promote_status(GlobalStatus::InitialClientLoadingWait);
}

fn on_initial_client_loading_wait(glm: &GlobalLifecycleManager) {
    glm.promote_status(GlobalStatus::GameRunning);
}

fn on_game_running(glm: &GlobalLifecycleManager) {
    glm.promote_status(GlobalStatus::GameEndProcess);
}
fn on_game_end_process(glm: &GlobalLifecycleManager) {
    glm.promote_status(GlobalStatus::WaitEndProcess);
}
fn on_wait_end_process(glm: &GlobalLifecycleManager) {
    glm.promote_status(GlobalStatus::RoomTerminated);
}
fn on_room_terminated(glm: &GlobalLifecycleManager) {
    // Exit this gracefully
    exit(0);
}
