use std::mem;

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    size: usize,
    queue: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        self.queue.enqueue(elem);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.size == 0 {
            return Err("Stack is empty");
        }

        for _ in 0..self.size - 1 {
            if let Ok(t) = self.queue.dequeue() {
                self.queue.enqueue(t);
            }
        }

        self.size -= 1;
        Ok(self.queue.dequeue().unwrap())
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = MyStack::<i32>::new();
        assert_eq!(stack.pop(), Err("Stack is empty"));
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Ok(3));
        assert_eq!(stack.pop(), Ok(2));
        stack.push(4);
        stack.push(5);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.pop(), Ok(5));
        assert_eq!(stack.pop(), Ok(4));
        assert_eq!(stack.pop(), Ok(1));
        assert_eq!(stack.pop(), Err("Stack is empty"));
        assert_eq!(stack.is_empty(), true);
    }
}
