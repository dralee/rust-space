/**
 * 自定义线程池
 * 2024.04.18 by dralee
 */


/* // version 1 
pub struct ThreadPool;

impl ThreadPool {
	/// Create a new ThreadPool.
	/// 
	/// The size is the number of threads in the pool.
	/// 
	/// # Panics
	/// 
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);
		ThreadPool
	}

	pub fn execute<F>(&self, f: F) 
	where F: FnOnce() + Send + 'static,
	{

	}
}*/

/* // version 2
use std::thread;

pub struct ThreadPool{
	threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
	/// Create a new ThreadPool.
	/// 
	/// The size is the number of threads in the pool.
	/// 
	/// # Panics
	/// 
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let mut threads = Vec::with_capacity(size);

		for _ in 0..size {
			// create some threads and store them in the vector
		}

		ThreadPool { threads }
	}

	pub fn execute<F>(&self, f: F) 
	where F: FnOnce() + Send + 'static,
	{

	}
}*/

/*
// version 3 工作单元

use std::thread;

// 使用工作单元
pub struct ThreadPool{
	workers: Vec<Worker>,
}

impl ThreadPool {
	/// Create a new ThreadPool.
	/// 
	/// The size is the number of threads in the pool.
	/// 
	/// # Panics
	/// 
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id));
		}

		ThreadPool { workers }
	}

	pub fn execute<F>(&self, f: F) 
	where F: FnOnce() + Send + 'static,
	{

	}
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}
impl Worker {
	fn new(id: usize) -> Worker {
		let thread = thread::spawn(|| {});
		Worker { id, thread }
	}
}
*/

/*
// 使用通道通信发送给worker,但消息只能多产单消
use std::{sync::mpsc,  thread};

// 使用通道通信
pub struct ThreadPool{
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
	/// Create a new ThreadPool.
	/// 
	/// The size is the number of threads in the pool.
	/// 
	/// # Panics
	/// 
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, receiver)); // value moved here, in previous iteration of loop
		}

		ThreadPool { workers, sender }
	}

	pub fn execute<F>(&self, f: F) 
	where F: FnOnce() + Send + 'static,
	{

	}
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}
impl Worker {
	fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
		let thread = thread::spawn(|| {
			receiver;
		});

		Worker { id, thread }
	}
}*/

// 解决多消问题，使用互斥
use std::{sync::{mpsc, Arc, Mutex},  thread};

// 使用通道通信
pub struct ThreadPool{
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
	/// Create a new ThreadPool.
	/// 
	/// The size is the number of threads in the pool.
	/// 
	/// # Panics
	/// 
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver)); // receiver变为互斥
		
		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		ThreadPool { workers, sender }
	}

	pub fn execute<F>(&self, f: F) 
	where F: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);

		self.sender.send(job).unwrap(); // 发送job到worker工作者线程
	}
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}
impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || loop {
			let job = receiver.lock().unwrap().recv().unwrap();

			println!("Worker {id} got a job; executing.");

			job();
		});

		Worker { id, thread }
	}
}