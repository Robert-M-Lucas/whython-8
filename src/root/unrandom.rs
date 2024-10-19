use std::collections::{HashMap, HashSet};
#[cfg(debug_assertions)]
use std::hash::RandomState;
#[cfg(debug_assertions)]
use std::mem::size_of;

/// In debug mode creates a deterministic `HashMap`, in release mode creates a normal one
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

/// Sets the `RandomState` of a `HashMap` to 0 to make it deterministic
/// 
/// Uses unsafe, unstable code - only use in debug mode!
#[cfg(debug_assertions)]
#[inline]
pub fn unrandom_hashmap<A, B>() -> HashMap<A, B> {
    let r: RandomState = unsafe { std::mem::transmute([0u8; size_of::<RandomState>()]) };

    HashMap::with_hasher(r)
}

/// In debug mode creates a deterministic `HashSet`, in release mode creates a normal one
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

/// Sets the `RandomState` of a `HashSet` to 0 to make it deterministic
///
/// Uses unsafe, unstable code - only use in debug mode!
#[cfg(debug_assertions)]
#[inline]
pub fn unrandom_hashset<A>() -> HashSet<A> {
    let r: RandomState = unsafe { std::mem::transmute([0u8; size_of::<RandomState>()]) };

    HashSet::with_hasher(r)
}
