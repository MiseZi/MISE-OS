#![no_std]

mod block_cache;
mod block_dev;
mod layout;
mod bitmap;


extern crate alloc;


pub const BLOCK_SZ: usize = 512;
