use std::{
    sync::{
        Arc,
        mpsc::Receiver,
        Mutex,
    },
    thread,
};

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    /// Creates a new thread that waits for a Job closure then executes it
    /// 
    /// # Arguments
    /// 
    /// * `id` - A usize that uniquely identifies the new worker
    /// 
    /// * `receiver` - A receiver from the ThreadPool to get Jobs from
    /// 
    /// # Returns
    /// 
    /// * The Job closure must return a Result<Job, anyhow::Error>
    pub fn new<F>(
        id: usize,
        receiver: Arc<Mutex<Receiver<F>>>
    ) -> Result<Worker, anyhow::Error>
    where
        F: FnOnce() + Send + 'static, 
    {
        println!("Creating Worker {id}");

        // spawn a new thread to wait jor a Job message from the thread pool
        let thread = thread::Builder::new()
        .spawn(move || loop {
            println!("Worker {id} is waiting for a job message");
            
            // get a lock on the mutex 
            match receiver.lock() {
                Ok(receiver_guard) => {
                    // wait to receive a Job
                    match receiver_guard.recv() {
                        Ok(job) => {
                            println!("Wokrer {id} got a job; executing.");
                            // execute the Job
                            job();
                        }
                        Err(recv_error) => {
                            println!("Worker {id} shutting down: {recv_error};");
                            break;
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Worker {id} error: {error}");
                    break;
                }
            };
        });

        return Ok(Worker {
            id,
            thread: Some(thread?),
        });
    }
}
