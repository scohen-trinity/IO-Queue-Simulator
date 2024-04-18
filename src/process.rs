use rand::Rng;

#[derive(Debug)]
pub enum ProcessType {
    Asynchronous,
    Synchronous,
}

#[derive(Debug)]
pub struct Process {
    pub duration: u64,
    pub process_type: ProcessType,
}

impl Process {
    pub fn generate_process() -> Self {
        let duration = rand::thread_rng().gen_range(1..=10);
        let process = rand::thread_rng().gen_range(0..=1);
        let mut process_type: ProcessType = ProcessType::Synchronous;
        if process == 1 {
            process_type = ProcessType::Asynchronous;
        }
        Self {
            duration,
            process_type,
        }
    }
}