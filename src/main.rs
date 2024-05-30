use std::collections::VecDeque;
use std::sync::{Arc,Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Message {
    content: String,
}

struct MessageQueue {
    queue: Arc<(Mutex<VecDeque<Message>>, Condvar)>,
}

impl MessageQueue {
    fn new() -> Self {
        MessageQueue {
            queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    fn produce(&self, message: Message) {
        let (lock,cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        queue.push_back(message);
        cvar.notify_one();
    }

    fn consume(&self) -> Message {
        let (lock, cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        while queue.is_empty() {
            queue = cvar.wait(queue).unwrap();
        }
        queue.pop_front().unwrap()
    }

    fn try_consume(&self) -> Option<Message> {
        let (lock, _cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        queue.pop_front()
    }
}

fn producer(queue: Arc<MessageQueue>, id: u32) {
    for i in 0..5 {
        let message = Message {
            content: format!("Message {} from producer {}", i, id),
        };
        println!("Producer {} sending: {:?}", id, message);
        queue.produce(message);
        thread::sleep(Duration::from_millis(100));
    }
}

fn consumer(queue: Arc<MessageQueue>, id: u32) {
    loop {
        if let message = queue.consume() {
            println!("Consumer {} received: {:?}", id, message);
        } else {
            break;
        }
        thread::sleep(Duration::from_millis(150));
    }
}

fn main() {
    let queue = Arc::new(MessageQueue::new());

       let mut handles = vec![];

    for i in 0..3 {
        let producer_queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            producer(producer_queue, i);
        });
        handles.push(handle);
    }

    for i in 0..3 {
        let consumer_queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            consumer(consumer_queue, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    

}


