// A simple fast LRU cache. It will use fixed capacity array size It provides `O(1)` insertion, and `O(n)`
//! lookup.  It does not require an allocator and can be used in `no_std` crates.

// This will cause to not load the standard library which we try in bare metal environments, more
// knowledge here https://docs.rust-embedded.org/book/intro/no-std.html
#![no_std]
#![deny(unsafe_code)]

use arrayvec::ArrayVec;
use core::{mem::replace, num::NonZero}; // Replaces the previous with the new with its reference to the old memory

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
            let old_entry = replace(self.entry(i), new_entry);
            self.push_front(i);
            Some(old_entry.val)
        } else {
            let i = self.entries.len() as u16;
            self.entries.push(new_entry);
            self.push_front(i);
            None
        }
    }

    pub fn entry(&mut self, i: u16) -> &mut Entry<T> {
        &mut self.entries[i as usize]
    }

    pub fn pop_back(&mut self) -> u16 {
        let new_tail = self.entry(self.tail).prev;
        replace(&mut self.tail, new_tail)
    }

    pub fn push_front(&mut self, i: u16) {
        if self.entries.len() == 1 {
            self.tail = i;
        } else {
            self.entry(i).next = self.head;
            self.entry(self.head).prev = i;
        }
        self.head = i;
    }

    // Returns the number of elements in the cache
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    // Returns if cache is empty or not
    pub fn is_empty(&self) -> bool {
        if self.len() == 0 {
            return true;
        }
        false
    }

    // Clears all the elements in cache
    pub fn clear(&mut self) {
        self.entries.clear()
    }
}

fn main() {}
