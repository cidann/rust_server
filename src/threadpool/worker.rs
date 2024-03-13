use std::{sync::{mpsc::Receiver, Arc, Mutex}, thread};
use super::Job;

pub struct Worker{
    id:usize,
    thread:thread::JoinHandle<()>
}


impl Worker {
    pub fn new(id:usize,receiver:Arc<Mutex<Receiver<Job>>>)->Worker{
        let thread=move ||{
            let receiver=receiver;
            loop {
                if let Ok(job)=consume_job(&receiver){
                    job();
                }
            };
        };
        Worker{
            id,
            thread:thread::spawn(thread)
        }
    }

}

fn consume_job(receiver:&Arc<Mutex<Receiver<Job>>>)->Result<Job,&'static str>{
    receiver
    .lock()
    .map_err(|_|"another thread paniced with lock")?
    .recv()
    .map_err(|_| "sender disconnected")
}
