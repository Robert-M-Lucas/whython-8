// ! Modified from https://users.rust-lang.org/t/iterator-need-to-identify-the-last-element/8836/3
// ! Specifically from users @Nemo157 and @reu

//! This module contains general helper traits.
use std::{iter, mem};

pub trait IdentifyFirstLast: Iterator + Sized {
    fn identify_first_last(self) -> IterFirstLast<Self>;
}

pub trait IdentifyFirst: Iterator + Sized {
    fn identify_first(self) -> IterFirst<Self>;
}

pub trait IdentifyLast: Iterator + Sized {
    fn identify_last(self) -> IterLast<Self>;
}

impl<I> IdentifyFirstLast for I
where
    I: Iterator,
{
    fn identify_first_last(self) -> IterFirstLast<Self> {
        IterFirstLast(true, self.peekable())
    }
}

impl<I> IdentifyFirst for I
where
    I: Iterator,
{
    fn identify_first(self) -> IterFirst<Self> {
        IterFirst(true, self)
    }
}

impl<I> IdentifyLast for I
where
    I: Iterator,
{
    fn identify_last(self) -> IterLast<Self> {
        IterLast(true, self.peekable())
    }
}

pub struct IterFirstLast<I>(bool, iter::Peekable<I>)
where
    I: Iterator;

pub struct IterFirst<I>(bool, I)
where
    I: Iterator;

pub struct IterLast<I>(bool, iter::Peekable<I>)
where
    I: Iterator;

impl<I> Iterator for IterFirstLast<I>
where
    I: Iterator,
{
    type Item = (bool, bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = mem::replace(&mut self.0, false);
        self.1.next().map(|e| (first, self.1.peek().is_none(), e))
    }
}

impl<I> Iterator for IterFirst<I>
where
    I: Iterator,
{
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = mem::replace(&mut self.0, false);
        self.1.next().map(|e| (first, e))
    }
}

impl<I> Iterator for IterLast<I>
where
    I: Iterator,
{
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|e| (self.1.peek().is_none(), e))
    }
}
