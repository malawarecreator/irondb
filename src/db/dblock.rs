use core::fmt;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dblock<A, B> {
    pub key: A,
    pub data: B
}

impl<A: Debug, B: Debug> fmt::Display for Dblock<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.key, self.data)
    }
}

impl<A, B> Dblock<A, B> {
    pub fn new(key: A, data: B) -> Self {
        Self {key, data}
    }
}