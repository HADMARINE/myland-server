use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub trait CyclicEvent {
    fn resolve(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_priority(&self) -> u32;
}
pub struct CyclicEventManager {
    event_queue: VecDeque<Box<dyn CyclicEvent + Send + Sync>>,
}

impl CyclicEventManager {
    pub fn enqueue(
        &self,
        event: Box<dyn CyclicEvent + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut executed_flag: bool = false;
        for (index, search_event) in self.event_queue.iter().enumerate() {
            if search_event.get_priority() > event.get_priority() {
                executed_flag = true;
                self.event_queue.insert(index, event);
            }
        }
        if !executed_flag {
            self.event_queue.push_back(event);
        }

        Ok(())
    }

    pub fn dequeue(
        &self,
    ) -> Result<Option<Box<dyn CyclicEvent + Send + Sync>>, Box<dyn std::error::Error>> {
        Ok(self.event_queue.pop_front())
    }

    pub fn consume_events_certain_priority(&self, priority: u32) {
        let join_handles: Vec<JoinHandle<Result<(), String>>> = vec![];
        for (index, event) in self.event_queue.iter_mut().enumerate() {
            if event.get_priority() == priority {
                let th_event = *event;
                let hdl = thread::spawn(move || {
                    let res = th_event.resolve();
                    match res {
                        Ok(_) => Ok(()),
                        Err(e) => return Err("Error occured while event consumption".to_string()),
                    }
                });
                join_handles.push(hdl);
                self.event_queue.drain(index..index);
            }
        }

        for handles in join_handles {
            let res = handles.join();
            match res {
                Ok(v) => match v {
                    Ok(v) => (),
                    Err(e) => {
                        println!("Error occoured! {}", e);
                    }
                },
                Err(e) => {
                    println!("Error occured!! {}", "NONE");
                }
            }
        }
    }

    pub fn consume_events_until_priority(&self, priority: u32) {
        let join_handles: Vec<JoinHandle<Result<(), String>>> = vec![];
        for (index, event) in self.event_queue.iter_mut().enumerate() {
            if event.get_priority() > priority {
                let th_event = *event;
                let hdl = thread::spawn(move || {
                    let res = th_event.resolve();
                    match res {
                        Ok(_) => Ok(()),
                        Err(e) => return Err("Error occured while event consumption".to_string()),
                    }
                });
                join_handles.push(hdl);
                self.event_queue.drain(index..index);
            } else {
                break;
            }
        }

        for handles in join_handles {
            let res = handles.join();
            match res {
                Ok(v) => match v {
                    Ok(v) => (),
                    Err(e) => {
                        println!("Error occoured! {}", e);
                    }
                },
                Err(e) => {
                    println!("Error occured!! {}", "NONE");
                }
            }
        }
    }

    pub fn consume_events_all(&self) {
        let join_handles: Vec<JoinHandle<Result<(), String>>> = vec![];
        for (index, event) in self.event_queue.iter_mut().enumerate() {
            let th_event = *event;
            let hdl = thread::spawn(move || {
                let res = th_event.resolve();
                match res {
                    Ok(_) => Ok(()),
                    Err(e) => return Err("Error occured while event consumption".to_string()),
                }
            });
            join_handles.push(hdl);
            self.event_queue.drain(index..index);
        }

        for handles in join_handles {
            let res = handles.join();
            match res {
                Ok(v) => match v {
                    Ok(v) => (),
                    Err(e) => {
                        println!("Error occoured! {}", e);
                    }
                },
                Err(e) => {
                    println!("Error occured!! {}", "NONE");
                }
            }
        }
    }
}
