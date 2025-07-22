use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
type ReceiverWorker = Arc<Mutex<mpsc::Receiver<Message>>>;
// Arc<Mutex<mspc::Receiver<Box<dyn FnBox + Send + 'static>>>>
type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}
impl Worker {
    fn new(id: usize, receiver: ReceiverWorker) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate", id);
                        break;
                    }
                };
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(number_of_pools: usize) -> Self {
        assert!(number_of_pools > 0); // return a Result here

        let mut workers = Vec::with_capacity(number_of_pools);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..number_of_pools {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // job: Box<dyn FnBox + Send + 'static>
        // FnBox => F: FnOnce() + Send + 'static
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

//Gracefull shutdown per worker
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down all workers");
        // force to all workers receive correctly the terminate message
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}
