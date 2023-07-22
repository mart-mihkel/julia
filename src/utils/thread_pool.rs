use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pub n_workers: usize,
    n_working: Arc<AtomicUsize>,
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(n_workers: usize) -> ThreadPool {
        let n_working = Arc::new(AtomicUsize::new(0));

        let (sender, receiver) = mpsc::channel();
        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..n_workers)
            .map(|_| Worker::new(
                Arc::clone(&receiver),
                Arc::clone(&n_working),
            ))
            .collect();

        ThreadPool { n_workers, n_working, workers, sender }
    }

    pub fn add_job<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .expect("ThreadPool::add_job() couldn't borrow sender as reference")
            .send(job)
            .expect("ThreadPool::add_job() couldn't send job");
    }

    pub fn has_work(&self) -> bool {
        self.n_working.load(Ordering::SeqCst) > 0
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Job>>>, n_working: Arc<AtomicUsize>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    n_working.fetch_add(1, Ordering::SeqCst);
                    job();
                    n_working.fetch_sub(1, Ordering::SeqCst);
                }
                Err(_) => break,
            }
        });

        let thread = Some(thread);

        Worker { thread }
    }
}

