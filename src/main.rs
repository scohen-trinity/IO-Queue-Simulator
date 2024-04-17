use std::collections::VecDeque;
use tokio::time::{sleep, Duration};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    let mut ready_q: Queue = Queue::new();
    let mut io_q: Queue = Queue::new();

    simulate(&mut ready_q, &mut io_q).await;
}

struct Queue {
    tasks: VecDeque<usize>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            tasks: VecDeque::new(),
        }
    }
    
    fn enqueue(&mut self, task: usize) {
        self.tasks.push_back(task);
    }

    fn dequeue(&mut self) -> Option<usize> {
        self.tasks.pop_front()
    }

    fn _is_empty(&mut self) -> bool {
        if self.tasks.len() == 0 {
            return true;
        }
        false
    }

    fn size(&mut self) -> usize {
        self.tasks.len()
    }
}

async fn simulate(ready_q: &mut Queue, io_q: &mut Queue) {
    while ready_q.size() != 0 && io_q.size() != 0 {

    }
}