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

    // Returns the first item in the cache that matches the predicate
    // Make it most recently used on hit
    pub fn find<F>(&mut self, pred: F) -> Option<&mut T>
    where
        F: FnMut(&T) -> bool,
    {
        if self.touch(pred) {
            self.front_mut()
        } else {
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
    #[inline] // https://nnethercote.github.io/perf-book/inlining.html
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    // Returns if cache is empty or not
    #[inline] // https://nnethercote.github.io/perf-book/inlining.html
    pub fn is_empty(&self) -> bool {
        if self.len() == 0 {
            return true;
        }
        false
    }

    // Clears all the elements in cache
    #[inline] // https://nnethercote.github.io/perf-book/inlining.html
    pub fn clear(&mut self) {
        self.entries.clear()
    }

    // Returns a mutable reference to the front entry in the list
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.entries.get_mut(self.head as usize).map(|e| &mut e.val)
    }

    // Touch a given entry, putting it first in the list.
    #[inline]
    fn touch_index(&mut self, i: u16) {
        if i != self.head {
            self.remove(i);
            self.push_front(i);
        }
    }

    // Remove an entry from the linked list.
    fn remove(&mut self, i: u16) {
        let prev = self.entry(i).prev;
        let next = self.entry(i).next;

        if i == self.head {
            self.head = next;
        } else {
            self.entry(prev).next = next;
        }

        if i == self.tail {
            self.tail = prev;
        } else {
            self.entry(next).prev = prev;
        }
    }

    // Touch the first item in the cache that matches the given predicate and marks it as recently
    // used, Returns true or false
    pub fn touch<F>(&mut self, mut pred: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        let mut iter = self.iter_mut();
        while let Some((i, val)) = iter.next() {
            if pred(val) {
                self.touch_index(i);
                return true;
            }
        }
        false
    }

    // Iterate mutably over the contents of this cache in order from most-recently-used to
    // least-recently-used.
    fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut {
            pos: self.head,
            cache: self,
        }
    }
}

struct IterMut<'a, T, const N: usize> {
    cache: &'a mut LRUCache<T, N>,
    pos: u16,
}

impl<'a, T, const N: usize> IterMut<'a, T, N> {
    fn next(&mut self) -> Option<(u16, &mut T)> {
        let index = self.pos;
        let entry = self.cache.entries.get_mut(index as usize)?;

        self.pos = if index == self.cache.tail {
            N as u16 // Point past the end of the array to signal we are done.
        } else {
            entry.next
        };
        Some((index, &mut entry.val))
    }
}

fn main() {}
