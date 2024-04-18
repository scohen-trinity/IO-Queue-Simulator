use tokio::time::Duration;
use crate::Queue;
use crate::ProcessType;

pub async fn simulate(ready_q: &mut Queue, io_q: &mut Queue) {
    println!("Simulation started");
    while ready_q.size() != 0 || io_q.size() != 0 {
        if let Some(process) = ready_q.dequeue() {
            match process.process_type {
                ProcessType::Synchronous => {
                    println!("Processing a synchronous task for {} milliseconds, task id: {}", process.duration, process.id);
                    std::thread::sleep(Duration::from_millis(process.duration))
                },
                ProcessType::Asynchronous => {
                    println!("Adding an io process to the io queue, task id: {}", process.id);
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(process.duration)).await;
                        println!("Finished async tasks, task id: {}", process.id);
                    });
                },
            }
        }
    }
    println!("Simulation finished");
}