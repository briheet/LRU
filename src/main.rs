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

impl<T, const N: usize> Default for LRUCache<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> LRUCache<T, N> {
    // create a empty cache
    pub const fn new() -> Self {
        assert!(N < u16::MAX as usize, "capacity overflow");
        LRUCache {
            entries: ArrayVec::new_const(),
            head: 0,
            tail: 0,
        }
    }

    // Insert given key in cache

    pub fn insert(&mut self, val: T) -> Option<T> {
        let new_entry = Entry {
            val,
            prev: 0,
            next: 0,
        };

        // If cache is full, replace the oldest entry
        if self.entries.is_full() {
            let i = self.pop_back();
            let old_entry = replace(self.entry(i), i);
        }
    }
}

fn main() {}
