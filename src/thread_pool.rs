use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::Thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Worker executes Job
struct Worker {
    /// Id of the worker
    id: usize,
    /// Thread executing the Job
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new Worker.
    /// # Arguments
    ///
    /// * `id` - The id of the created Worker
    /// * `receiver` - The receiver used by the worker to get the job to execute
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}

/// Create threads and dispatch closures to be executed on their workers
pub struct ThreadPool {
    workers: Vec<Worker>,
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
    pub fn new(n_threads: usize) -> ThreadPool {
        assert!(n_threads > 0);

        // Create a channel to send a job between the thread pool and their workers.
        let (sender, receiver) = mpsc::channel();

        // Receiver is meant to be used by only one consumer at a time, therefore we have to use Arc
        // pointer and Mutex to allow each thread to have pointer to the receiver
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(n_threads);

        for id in 0..n_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Execute f
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
