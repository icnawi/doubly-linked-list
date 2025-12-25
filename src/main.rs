use crate::{deque::Deque, memory::Memory, node::Node};

mod deque;
mod memory;
mod node;

fn main() {
    let mut memory = Memory::<Node<i32>>::new();
    let mut deque = Deque::<i32>::new();

    // 1st Push Batch: Adding to the End
    for i in 0..10 {
        deque.push_back(&mut memory, i);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);

    // 2nd Push Batch: Adding to the Start
    for i in 10..20 {
        deque.push_front(&mut memory, i);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);

    // 3rd Push Batch: Adding to the Start
    for i in 0..10 {
        deque.push_front(&mut memory, i);
    }
    memory.debug_print();
    deque.debug_print(&memory);

    // 4th Push Batch: Adding to the Start
    for i in 0..10 {
        deque.push_front(&mut memory, i);
    }
    memory.debug_print();
    deque.debug_print(&memory);

    // 1st Pop: Start
    deque.pop_front(&mut memory);
    memory.debug_print();
    deque.debug_print(&memory);

    // 2nd Pop: End
    deque.pop_back(&mut memory);
    memory.debug_print();
    deque.debug_print(&memory);

    // 3rd Pop: End
    deque.pop_back(&mut memory);
    memory.debug_print();
    deque.debug_print(&memory);

    // 4th Pop: Start
    deque.pop_front(&mut memory);
    memory.debug_print();
    deque.debug_print(&memory);
}
