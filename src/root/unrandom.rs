use std::hash::RandomState;
use std::{mem, ptr, slice};
use std::collections::{HashMap, HashSet};

#[inline]
pub fn new_hashmap<A, B>() -> HashMap<A, B> {
    #[cfg(debug_assertions)]
    {
        unrandom_hashmap()
    }
    #[cfg(not(debug_assertions))]
    {
        HashMap::new()
    }
}

#[inline]
pub fn unrandom_hashmap<A, B>() -> HashMap<A, B> {
    let mut r = RandomState::new();
    let p: *mut RandomState = &mut r;
    let p: *mut u8 = p as *mut u8;
    let s: &mut [u8] = unsafe {
        slice::from_raw_parts_mut(p, mem::size_of::<RandomState>())
    };
    unsafe {
        for s in &mut *s {
            let p: *const u8 = s;
            let p: *mut u8 = p as *mut u8;
            ptr::write_volatile(p, 0u8);
        }
    }

    HashMap::with_hasher(r)
}

#[inline]
pub fn new_hashset<A>() -> HashSet<A> {
    #[cfg(debug_assertions)]
    {
        unrandom_hashset()
    }
    #[cfg(not(debug_assertions))]
    {
        HashSet::new()
    }
}

#[inline]
pub fn unrandom_hashset<A>() -> HashSet<A> {
    let mut r = RandomState::new();
    let p: *mut RandomState = &mut r;
    let p: *mut u8 = p as *mut u8;
    let s: &mut [u8] = unsafe {
        slice::from_raw_parts_mut(p, mem::size_of::<RandomState>())
    };
    unsafe {
        for s in &mut *s {
            let p: *const u8 = s;
            let p: *mut u8 = p as *mut u8;
            ptr::write_volatile(p, 0u8);
        }
    }

    HashSet::with_hasher(r)
}