use std::{
    io,
    sync::{self, Arc, Mutex},
    thread,
};

/// # ThreadPool
///
/// A struct used for defining a limited number of threads available for use.
/// This is mainly used for allowing multiple requests to be handled at once, but with a limited suceptibility of DOS attacks
pub struct ThreadPool {
    _thread_limit: usize,
    workers: Vec<Worker>,
    sender: Option<sync::mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// # ThreadPool::new
    ///
    /// Creates a new instance of a thread pool with a specified thread limit
    ///
    /// ## Arguments
    ///
    /// * `thread_limit` - The maximum number of threads that can be used at once
    ///
    /// ## Example
    ///
    /// ```rs
    /// use carola::thread_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    /// ```
    ///
    /// ## Panics
    ///
    /// This function will panic if the thread limit is 0
    pub fn new(thread_limit: usize) -> ThreadPool {
        assert!(thread_limit > 0);
        let (sender, reciever) = sync::mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(thread_limit);
        for i in 0..thread_limit {
            workers.push(Worker::spawn(i, Arc::clone(&reciever)).unwrap());
        }
        Self {
            _thread_limit: thread_limit,
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F: FnOnce() + Send + 'static>(&self, function: F) {
        self.sender.as_ref().unwrap().send(Box::new(function)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        std::mem::drop(self.sender.take().unwrap());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn spawn(
        id: usize,
        reciever: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>,
    ) -> Result<Self, io::Error> {
        let builder = thread::Builder::new();
        let thread = builder.spawn(move || loop {
            let message = reciever.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    // Sender has been dropped, indicating that the thread pool is shutting down
                    break;
                }
            }
        })?;

        let thread = Some(thread);
        
        Ok(Self { _id: id, thread })
    }
}
