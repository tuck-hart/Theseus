#![no_std]

extern crate memuse;

pub fn main(){
    let res = memuse::memuse(0, memuse::MemoryType::HEAP);
}