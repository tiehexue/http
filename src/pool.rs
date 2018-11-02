use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

enum Message {
  NewJob(Job),
  Terminate
}

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<Self>) {
    (*self)();
  }
}

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
  sender: mpsc::Sender<Message>,
  _threads: Vec<Option<JoinHandle<()>>>
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    let mut _threads = Vec::with_capacity(size);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    for _ in 0..size {
      let receiver = Arc::clone(&receiver);

      _threads.push(Some(thread::spawn(move || {

        loop {
          println!("ID in thread pool: {:#?}", thread::current().id());
          let msg: Message = receiver.lock().unwrap().recv().unwrap();

          match msg {
            Message::NewJob(job) => {
              job.call_box();
            },
            Message::Terminate => {
              break;
            }
          }
        }
      })));
    }

    ThreadPool {sender, _threads}
  }

  pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
    self.sender.send(Message::NewJob(Box::new(f))).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {

    println!("Sending terminates ...");
    for i in 0..self._threads.len() {
      println!("Sending terminates ... ({}).", i);
      self.sender.send(Message::Terminate).unwrap();
    }

    for t in &mut self._threads {
      t.take().unwrap().join().unwrap();
      println!("Gracefully terminated ...");
    }
  }
}
