#![no_std]

use task::TaskRef;

extern crate log;
extern crate task;
extern crate heap;

#[derive(Debug, PartialEq)]
pub enum MemoryType{
    HEAP,
    STACK,
}

pub fn memuse(task_id: usize, mem_type: MemoryType) -> Result<TaskRef, &'static str>{

    let res = match task::get_task(task_id) {
        None => Err("Task not found"),
        Some(task) => {
            let mut memsize = 0;
            if mem_type == MemoryType::HEAP {
                memsize = task.get_memsize();
            } else {
                memsize = task.get_stacksize();
            }
            
            log::debug!("memuse called with task id: {}, memory type: {:?}. Got memory size of: {:?} Bytes", task_id, mem_type, memsize);
            Ok(task)
        }
    };

    return res;
}