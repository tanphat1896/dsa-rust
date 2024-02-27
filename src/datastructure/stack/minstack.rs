use std::{cmp::min, collections::LinkedList};

#[derive(Debug)]
pub struct Node(pub i32, pub i32);

struct MinStack {
    data: LinkedList<Node>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            data: LinkedList::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let min_v = self.data.front().map_or(val, |v| v.1);
        self.data.push_front(Node(val, min(min_v, val)))
    }

    fn pop(&mut self) {
        self.data.pop_front();
    }

    fn top(&self) -> i32 {
        self.data.front().map_or(0, |v| v.0)
    }

    fn get_min(&self) -> i32 {
        self.data.front().map_or(0, |v| v.1)
    }
}

#[cfg(test)]
mod test {
    use crate::datastructure::stack::minstack::MinStack;

    #[test]
    fn test() {
        let mut m = MinStack::new();
        m.push(-2);
        m.push(0);
        m.push(-3);
        assert_eq!(-3, m.get_min());

        m.pop();
        assert_eq!(0, m.top());
        assert_eq!(-2, m.get_min());
    }
}
