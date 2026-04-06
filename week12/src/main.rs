#![allow(dead_code)]

use std::fmt;

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

fn main() {
    println!("Lab 12: Generic Stack Implementation\n");

    println!("=== Integer Stack ===");
    let mut int_stack = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    println!("Stack: {}", int_stack);
    println!("Popped: {:?}", int_stack.pop());
    println!("Peek: {:?}", int_stack.peek());
    println!();

    println!("=== String Stack ===");
    let mut string_stack = Stack::new();
    string_stack.push(String::from("Rust"));
    string_stack.push(String::from("is"));
    string_stack.push(String::from("awesome"));
    println!("Stack: {}", string_stack);

    println!("\nIterating (LIFO order):");
    for item in string_stack {
        println!("  {}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_push_increases_len() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_pop_returns_lifo_order() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek_does_not_remove() {
        let mut stack = Stack::new();
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_peek_empty_stack() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_stack_of_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("hello"));
        stack.push(String::from("world"));
        assert_eq!(stack.pop(), Some(String::from("world")));
    }

    #[test]
    fn test_stack_of_floats() {
        let mut stack = Stack::new();
        stack.push(3.14);
        stack.push(2.71);
        assert_eq!(stack.pop(), Some(2.71));
    }

    #[test]
    fn test_is_empty_after_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_display_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(format!("{}", stack), "[]");
    }

    #[test]
    fn test_display_with_items() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(format!("{}", stack), "[1, 2, 3]");
    }

    #[test]
    fn test_iterator() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.next(), Some(3));
        assert_eq!(stack.next(), Some(2));
        assert_eq!(stack.next(), Some(1));
        assert_eq!(stack.next(), None);
    }

    #[test]
    fn test_for_loop_lifo_order() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);

        let mut results = Vec::new();
        for item in stack {
            results.push(item);
        }
        assert_eq!(results, vec![2, 1]);
    }

    #[test]
    fn test_iterator_with_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("first"));
        stack.push(String::from("second"));

        let collected: Vec<String> = stack.collect();
        assert_eq!(collected, vec![String::from("second"), String::from("first")]);
    }

    #[test]
    fn test_polymorphism() {
        let mut int_stack = Stack::new();
        int_stack.push(1);
        int_stack.push(2);
        assert_eq!(int_stack.pop(), Some(2));

        let mut string_stack = Stack::new();
        string_stack.push(String::from("hello"));
        string_stack.push(String::from("world"));
        assert_eq!(string_stack.pop(), Some(String::from("world")));

        let mut float_stack = Stack::new();
        float_stack.push(3.14);
        float_stack.push(2.71);
        assert_eq!(float_stack.pop(), Some(2.71));
    }
}