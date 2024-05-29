use std::collections::VecDeque;

#[derive(Debug)]
struct Message {
    content: String,
}

struct MessageQueue{
    queue: VecDeque<Message>,
}

impl MessageQueue{
    fn new()->Self{
        MessageQueue{
            queue: VecDeque::new(),
        }
    }

    fn produce(&mut self, message Message){
        self.queue.push_back(message);
    }

    fn consume(&mut self)-> Option<Message>{
        self.queue.pop_front();
    }
}

fn main() {
    let mut queue = MessageQueue::new();
    
    queue.produce(Message{
        content: String::from("Message pushed to the queue");
    });

    if let Some(msg) = queue.consume() {
        println!("Consumed: {:?}", msg);
    }

}
