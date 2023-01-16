use std::{
    io,
    sync::{self, Arc, Mutex},
    thread,
};

/// # ThreadPool
///
/// A struct used for defining a limited number of threads available for use
pub struct ThreadPool {
    thread_limit: usize,
    workers: Vec<Worker>,
    sender: sync::mpsc::Sender<Job>,
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
            thread_limit,
            workers,
            sender,
        }
    }
    pub fn execute<F: FnOnce() + Send + 'static>(&self, function: F) {
        self.sender.send(Box::new(function)).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn spawn(
        id: usize,
        reciever: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>,
    ) -> Result<Self, io::Error> {
        let builder = thread::Builder::new();
        let thread = builder.spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            job();
        })?;
        Ok(Self { id, thread })
    }
}
