use std::thread::{self, Thread};
use std::sync::{Arc, Mutex, mpsc};

struct Worker {
    id: usize,
    work: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let work = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} executing job ...", id);
            job()
        });

        Worker { id , work }
    }
}

pub struct ThreadPool{
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool 
    {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut _workers = Vec::with_capacity(size);
        for id in 0..size
        {
            _workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { _workers , sender}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
