mod queue;
mod process;
mod simulator;

use queue::Queue;
use crate::simulator::simulate;
use process::{Process, ProcessType};

#[tokio::main]
async fn main() {
    let mut ready_q: Queue = Queue::new();
    let mut io_q: Queue = Queue::new();

    for i in 0..=72 {
        ready_q.enqueue(Process::generate_process(i));
    }

    simulate(&mut ready_q, &mut io_q).await;
}