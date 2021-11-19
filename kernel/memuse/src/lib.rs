#![no_std]

use task::TaskRef;

extern crate log;
extern crate task;

#[derive(Debug)]
pub enum MemoryType{
    HEAP,
    STACK,
}

pub fn memuse(task_id: usize, mem_type: MemoryType) -> Result<TaskRef, &'static str>{
    log::debug!("memuse called with task id: {}, memory type: {:?}", task_id, mem_type);
    let task_option = task::get_task(task_id);

    let res = match task_option {
        None => Err("Task not found"),
        Some(task) => Ok(task)
    };

    return res;
}