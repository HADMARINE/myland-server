use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

lazy_static::lazy_static! {
    static ref CYCLIC_EVENT_QUEUE : Arc<Mutex<VecDeque<Box<dyn CyclicEvent + Send + Sync>>>> = Arc::new(Mutex::from(VecDeque::new()));
}

pub trait CyclicEvent {
    fn resolve(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_priority(&self) -> u32;
}

pub fn enqueue(
    event: Box<dyn CyclicEvent + Send + Sync>,
) -> Result<(), Box<dyn std::error::Error>> {
    // CYCLIC_EVENT_QUEUE.lock()?.push_back(event);
    let mut locked_event_queue = CYCLIC_EVENT_QUEUE.lock()?;
    let mut executed_flag: bool = false;
    for (index, search_event) in locked_event_queue.iter().enumerate() {
        if search_event.get_priority() > event.get_priority() {
            executed_flag = true;
            locked_event_queue.insert(index, event);
        }
    }
    if !executed_flag {
        locked_event_queue.push_back(event);
    }

    Ok(())
}

pub fn dequeue() -> Result<Option<Box<dyn CyclicEvent + Send + Sync>>, Box<dyn std::error::Error>> {
    Ok(CYCLIC_EVENT_QUEUE.lock()?.pop_front())
}
