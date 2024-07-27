use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

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
    let r: RandomState = unsafe { std::mem::transmute([0u8; size_of::<RandomState>()]) };

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
    let r: RandomState = unsafe { std::mem::transmute([0u8; size_of::<RandomState>()]) };

    HashSet::with_hasher(r)
}
