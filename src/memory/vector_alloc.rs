
//! memory/vector_alloc.rs - Reiner Rust-Allokator

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
// In a real scenario, we'd use a more sophisticated data structure.
// For this example, a simple bump allocator will suffice.

const HEAP_SIZE: usize = 1024 * 64; // 64 KB heap
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut NEXT: usize = 0;

pub struct VectorAllocator;

unsafe impl GlobalAlloc for VectorAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = NEXT;
        let align = layout.align();
        let start_aligned = (start + align - 1) & !(align - 1);

        if start_aligned + layout.size() > HEAP_SIZE {
            return null_mut(); // Out of memory
        }

        NEXT = start_aligned + layout.size();
        &mut HEAP[start_aligned] as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // This simple bump allocator doesn't support deallocation.
    }
}

#[global_allocator]
static ALLOCATOR: VectorAllocator = VectorAllocator;
