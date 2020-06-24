
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

trait FnBox{
    fn call(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call(self: Box<Self>) {
        (*self)()
    }
}

pub type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize)->ThreadPool{
        let (tx,rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(
                Worker::new(Arc::clone(&rx))
            );
        }
        ThreadPool{
            workers,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F : FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        for i in 0..self.workers.len(){
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker{
    pub fn new(rx: Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{
        let thread = thread::spawn(move ||{
            loop {
                let msg = rx.lock().unwrap().recv().unwrap();
                match msg {
                    Message::NewJob(job)=>{
                        job.call();
                    },
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });
        Worker{
            thread: Some(thread)
        }
    }
}