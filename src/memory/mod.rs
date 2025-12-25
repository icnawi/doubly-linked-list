use std::fmt::Debug;

pub struct Memory<T> {
    buffer: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T: Debug> Memory<T> {
    pub fn new() -> Self {
        Self {
            buffer: Vec::<Option<T>>::new(),
            free: Vec::new(),
        }
    }

    pub fn debug_print(&self) {
        print!("buffer: ");
        for _ in self.buffer.iter() {
            print!("* ");
        }
        println!("");

        print!("free: ");
        for x in self.free.iter() {
            print!("{:?} ", x);
        }
        println!("");
        println!("slots: {:?}", self.free_slots());
        println!("bytes: {:?}", self.free_bytes());
    }

    pub fn alloc(&mut self, value: T) -> usize {
        if let Some(index) = self.free.pop() {
            self.buffer[index] = Some(value);
            index
        } else {
            self.buffer.push(Some(value));
            self.buffer.len() - 1
        }
    }

    pub fn dealloc(&mut self, pointer: usize) {
        self.free.push(pointer);
    }

    pub fn deref(&self, pointer: usize) -> Option<&T> {
        self.buffer[pointer].as_ref()
    }

    pub fn deref_mut(&mut self, pointer: usize) -> Option<&mut T> {
        self.buffer[pointer].as_mut()
    }

    fn free_slots(&self) -> usize {
        self.free.len() + (self.buffer.capacity() - self.buffer.len())
    }

    fn free_bytes(&self) -> usize {
        self.free_slots() * std::mem::size_of::<T>()
    }
}
