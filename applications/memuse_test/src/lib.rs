#![no_std]

extern crate memuse;
extern crate alloc;
extern crate task;

use alloc::vec;
use task::get_my_current_task_id;

pub fn test_stack(n:usize, id:usize) -> usize{
    let res = memuse::memuse(id, memuse::MemoryType::STACK);
    let v = 30;
    if n == 0 {
        0
    } else {
        test_stack(n - 1, id) + 1
    }
}

pub fn main() -> usize{
    let id:usize = match get_my_current_task_id(){
        Some(id) => id,
        None => {return 0;}
    };
    let res = memuse::memuse(id, memuse::MemoryType::HEAP);
    let v = vec![50, 50, 50];
    let res = memuse::memuse(id, memuse::MemoryType::HEAP);
    {
        let v = vec![50, 50, 50, 50];
        let res = memuse::memuse(id, memuse::MemoryType::HEAP);
    }
    let res = memuse::memuse(id, memuse::MemoryType::HEAP);
    // let res = memuse::memuse(id, memuse::MemoryType::STACK);
    test_stack(10, id);
    0
}