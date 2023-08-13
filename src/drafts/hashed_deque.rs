use std::collections::{HashSet, LinkedList};

#[derive(Debug)]
pub struct HashedDeque<T> {
    list: LinkedList<T>,
    set: HashSet<T>,
}

impl<T: std::hash::Hash + std::cmp::Eq + Clone> HashedDeque<T> {
    pub fn new() -> Self {
        HashedDeque {
            list: LinkedList::new(),
            set: HashSet::new(),
        }
    }

    pub fn push_back(&mut self, value: T) {
        if self.set.insert(value.clone()) {
            self.list.push_back(value);
        }
    }

    pub fn push_front(&mut self, value: T) {
        if self.set.insert(value.clone()) {
            self.list.push_front(value);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let value = self.list.pop_back()?;
        self.set.remove(&value);
        Some(value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let value = self.list.pop_front()?;
        self.set.remove(&value);
        Some(value)
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl<T: std::hash::Hash + std::cmp::Eq + Clone> HashedDeque<T> {
    pub fn append(&mut self, other: &mut HashedDeque<T>) {
        while let Some(value) = other.pop_front() {
            self.push_back(value);
        }
    }
}
