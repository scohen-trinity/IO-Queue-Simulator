use crate::process::Process;
use std::collections::VecDeque;

pub struct Queue {
    tasks: VecDeque<Process>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            tasks: VecDeque::new(),
        }
    }
    
    pub fn enqueue(&mut self, task: Process) {
        self.tasks.push_back(task);
    }

    pub fn dequeue(&mut self) -> Option<Process> {
        self.tasks.pop_front()
    }

    fn _is_empty(&mut self) -> bool {
        if self.tasks.len() == 0 {
            return true;
        }
        false
    }

    pub fn size(&mut self) -> usize {
        self.tasks.len()
    }
}