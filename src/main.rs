mod queue;
mod process;

use queue::Queue;
use process::{Process, ProcessType};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut ready_q: Queue = Queue::new();
    let mut io_q: Queue = Queue::new();

    for _ in 0..=5 {
        ready_q.enqueue(Process::generate_process());
    }

    simulate(&mut ready_q, &mut io_q).await;
}

async fn simulate(ready_q: &mut Queue, io_q: &mut Queue) {
    println!("Simulation started");
    while ready_q.size() != 0 || io_q.size() != 0 {
        if let Some(process) = ready_q.dequeue() {
            match process.process_type {
                ProcessType::Synchronous => {
                    println!("Processing a synchronous task for {} milliseconds", process.duration);
                    std::thread::sleep(Duration::from_millis(process.duration))
                },
                ProcessType::Asynchronous => {
                    println!("Adding an io process to the io queue");
                    io_q.enqueue(process)
                },
            }
        }
    }
    println!("Simulation finished");
}