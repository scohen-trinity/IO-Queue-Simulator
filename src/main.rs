use std::collections::VecDeque;
use tokio::time::{sleep, Duration};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    let mut q: Queue = Queue::new();
    let mut devices = vec![Device::new(1), Device::new(2)];
    
    for i in 1..=5 {
        q.enqueue(i);
    }
    simulate(&mut q, &mut devices).await;
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

    fn is_empty(&mut self) -> bool {
        if self.tasks.len() == 0 {
            return true;
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
struct Device {
    id: usize,
}

impl Device {
    fn new(id: usize) -> Self {
        Self {
            id,
        }
    }

    async fn process_task(&self, task: usize) {
        println!("Device {} is processing task {}", self.id, task);
        // simulating a computation process
        self.simulate_computation().await;
        // simulating an I/O operation
        self.simulate_io().await;
        println!("Device {} finished processing task {}", self.id, task);
    }

    async fn simulate_computation(&self) {
        tokio::spawn(async {
            sleep(Duration::from_secs(1)).await;
        }).await.unwrap();
    }

    async fn simulate_io(&self) {
        let _file = File::create(format!("task_{}.txt", self.id)).await.unwrap();
    }
}

async fn simulate(queue: &mut Queue, devices: &mut Vec<Device>) {
    loop {
        if let Some(task) = queue.dequeue() {
            if let Some(device) = devices.pop() {
                let cloned_device = device.clone();
                tokio::spawn(async move {
                    cloned_device.process_task(task).await
                });
                devices.push(device);
            } else {
                queue.enqueue(task);
            }
        }
    }
}