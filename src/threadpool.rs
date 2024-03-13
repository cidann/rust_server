use self::worker::Worker;
use std::{f32::consts::E, net::TcpStream, sync::{mpsc::{self, Sender}, Arc, Mutex}};

mod worker;

pub struct ThreadPool{
    workers:Vec<worker::Worker>,
    sender:Sender<Job>
}

type Job=Box<dyn FnOnce()->()+Send+ 'static>;

impl ThreadPool {
    pub fn new(size:usize)->ThreadPool{
        if size==0{
            panic!("ThreadPool needs at least 1 worker")
        }
        let mut workers=Vec::with_capacity(size);
        let (tx,rx)=mpsc::channel();
        let rx_ref=Arc::new(Mutex::new(rx));
        
        for i in 0..size{
            workers.push(Worker::new(i,rx_ref.clone()));
        }

        ThreadPool{
            workers,
            sender:tx
        }
    }

    pub fn execute(&self,job:Job)->Result<(),&'static str>{
        match self.sender.send(job) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to execute job"),
        }
    }
}