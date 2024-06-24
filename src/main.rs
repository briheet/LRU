// A simple fast LRU cache. It will use fixed capacity array size It provides `O(1)` insertion, and `O(n)`
//! lookup.  It does not require an allocator and can be used in `no_std` crates.

// This will cause to not load the standard library which we try in bare metal environments, more
// knowledge here https://docs.rust-embedded.org/book/intro/no-std.html
#![no_std]
#![deny(unsafe_code)]

use arrayvec::ArrayVec;
use core::mem::replace; // Replaces the previous with the new with its reference to the old memory

#[derive(Debug, Clone)]

pub struct LRUCache<T, const N: usize> {
    // Recent entry is at index head
    entries: ArrayVec<Entry<T>, N>,
    // Index of the first entry
    head: u16,
    // Index of the last entry
    tail: u16,
}

#[derive(Debug, Clone)]
struct Entry<T> {
    val: T,
    prev: u16,
    next: u16,
}

fn main() {}
