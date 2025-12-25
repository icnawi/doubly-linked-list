use std::fmt::Debug;
use std::marker::PhantomData;

use crate::memory::Memory;
use crate::node::Node;

pub struct Deque<T> {
    start: Option<usize>,
    end: Option<usize>,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug> Deque<T> {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            phantom: Default::default(),
        }
    }

    pub fn debug_print(&self, memory: &Memory<Node<T>>) {
        let mut iter = self.start;
        print!("deque: |");
        while let Some(node_ref) = iter {
            print!("{:?} ", memory.deref(node_ref).unwrap().value);
            iter = memory.deref(node_ref).unwrap().next;
        }
        print!("|");
        println!("");
    }

    pub fn push_front(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(start_node) = self.start {
            memory.deref_mut(new_node).unwrap().next = Some(start_node);
            memory.deref_mut(new_node).unwrap().prev = Some(new_node);
            self.start = Some(new_node);
        } else {
            assert!(self.end.is_none());
            self.start = Some(new_node);
            self.end = Some(new_node);
        }
    }

    pub fn push_back(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(end_node) = self.end {
            memory.deref_mut(new_node).unwrap().prev = Some(end_node);
            memory.deref_mut(new_node).unwrap().next = Some(new_node);
            self.end = Some(new_node);
        } else {
            assert!(self.start.is_none());
            self.start = Some(new_node);
            self.end = Some(new_node);
        }
    }

    pub fn pop_front(&mut self, memory: &mut Memory<Node<T>>) {
        if let Some(start_ref) = self.start {
            self.start = memory.deref(start_ref).unwrap().next;
            memory.dealloc(start_ref);

            if self.start.is_none() {
                self.end = None;
            }
        }
    }

    pub fn pop_back(&mut self, memory: &mut Memory<Node<T>>) {
        if let Some(end_ref) = self.end {
            self.end = memory.deref(end_ref).unwrap().prev;
            memory.dealloc(end_ref);

            if self.end.is_none() {
                self.start = None;
            }
        }
    }
}
