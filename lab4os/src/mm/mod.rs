pub mod heap_allocator;
mod address;
mod page_table;
mod frame_allocator;
mod memory_set;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};

use crate::debug;


pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}