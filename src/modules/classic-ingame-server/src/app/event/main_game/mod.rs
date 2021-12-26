use std::{collections::HashMap, process::exit, time::Instant};

use self::{
    constants::USER_CONNECT_WAIT_TIME_SEC,
    implementations::main_loop::main_loop,
    lifecycle_manager::global::{GlobalLifecycleManager, GlobalStatus},
};

use super::EventMapType;

pub mod constants;
pub mod cyclic_event_queue;
pub mod event_handler;
pub mod game_manager;
pub mod implementations;
pub mod lifecycle_manager;
pub mod structs;
pub mod transformers;

// Event Handlers
pub fn get_config() -> EventMapType {
    let mut map: EventMapType = HashMap::new();
    map.insert("user_ready".to_string());
    map.insert("client_ready".to_string());

    map.insert("register".to_string());
    map.insert("relogin".to_string());

    map.insert("start_auction".to_string());
    map.insert("register_auction".to_string());

    map.insert("get_lobby_list".to_string());
    map.insert("select_lobby".to_string());

    map.insert("get_loan_list".to_string());
    map.insert("select_loan".to_string());

    map.insert("recruit_spy".to_string());
    map.insert("select_spy".to_string());

    map.insert("queue_construction".to_string());
    map.insert("queue_reclamation".to_string());

    map.insert("".to_string());
    map.insert("queue_reclamation".to_string());

    map
}

// Game Lifecycle handlers
pub fn start_game() {
    let mut glm = GlobalLifecycleManager::new();
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
    glm.kickoff();
}

fn on_wait_user(glm: &GlobalLifecycleManager) {
    let start_time = Instant::now();
    while game_manager::WAITING_USERS.len() == 0 {
        let elapsed = start_time.elapsed().as_secs();
        if elapsed > USER_CONNECT_WAIT_TIME_SEC {
            break;
        }
    }
    glm.promote_status(GlobalStatus::WaitUserReady);
}

fn on_wait_user_ready(glm: &GlobalLifecycleManager) {
    while game_manager::REGISTERED_USERS.len() == game_manager::USER_READY_LIST.len() {}
    glm.promote_status(GlobalStatus::InitialServerLoading);
}

fn on_initial_server_loading(glm: &GlobalLifecycleManager) {
    glm.promote_status(GlobalStatus::InitialClientLoadingWait);
}

fn on_initial_client_loading_wait(glm: &GlobalLifecycleManager) {
    glm.promote_status(GlobalStatus::GameRunning);
}

fn on_game_running(glm: &GlobalLifecycleManager) {
    main_loop();
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
