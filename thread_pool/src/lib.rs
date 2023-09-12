use std::sync::{
    Arc,
    mpsc,
    Mutex,
};

use anyhow::anyhow;

mod worker;
use self::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The pool_size is the number of threads in the returned pool.
    ///
    /// pool_size must be greater than 0.
    pub fn new(pool_size: usize) -> Result<ThreadPool, anyhow::Error> {
        if pool_size == 0 {
            return Err(anyhow!("Thread pool cannot have size 0"));
        }

        // create a channel to send jobs to workers
        let (job_sender, receiver) = mpsc::channel();

        // create a counted refrence of a mutual exclusion so all workers can share a job receiver
        let job_receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::<Worker>::with_capacity(pool_size);
        for id in 0..pool_size {
            // create workers and give them a clone of the receiver
            let worker = match Worker::new(id, job_receiver.clone()) {
                Ok(worker) => worker,
                Err(worker_error) => return Err(anyhow!("Could not create thread pool: {worker_error}")),
            };
            workers.push(worker);
        }

        return Ok(ThreadPool {
            workers,
            sender: Some(job_sender),
        });
    }

    /// Sends the Job closure to an avliable worker. If no workers 
    /// are available the Job will stay in the channel untill a worker
    /// receives the Job or the channel is closed
    /// 
    /// # Arguments
    /// `job` - A static lifetime closure that can be sent across threads
    pub fn execute<F>(&self, job: F) -> Result<(), anyhow::Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);

        return match &self.sender {
            Some(sender) => sender.send(job).map_err(|send_error| anyhow!("ThreadPool couldn't send job to worker: {send_error}")),
            None => Err(anyhow!("No sender in thread pool. Was it dropped?"))
        };
    }
}

impl Drop for ThreadPool {
    /// Joins all worker's thread JoinHandles
    fn drop(&mut self) {
        println!("Thread pool is closing the channel");
        core::mem::drop(self.sender.take());

        for worker in self.workers.iter_mut() {
            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Ok(_) => println!("Thread pool joining worker {}'s thread", worker.id),
                    Err(error) => eprintln!("Error while joining worker thread: {:?}", error)
                }
            }
        }

        println!("All worker threads joined main thread.");
    }
}