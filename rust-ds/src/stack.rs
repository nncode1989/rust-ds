#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stack_empty() {
        let mut stack: Stack<i16> = Stack { items: vec![] };
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_stack_non_empty() {
        let mut stack: Stack<i32> = Stack { items: vec![] };
        stack.push(10);
        stack.push(11);
        stack.push(12);
        assert_eq!(3, stack.size());
        assert_eq!(false, stack.is_empty());
        assert_eq!(Some(&12), stack.top());
        assert_eq!(Some(12), stack.pop());
        assert_eq!(Some(11), stack.pop());
        assert_eq!(Some(10), stack.pop());
        assert_eq!(None, stack.pop());
    }
}

/// Stack Trait.
pub trait StackTrait {
    type Item;
    /// Push item:Item to the top of stack
    fn push(&mut self, i: Self::Item);
    /// Remove and Return item:Item from the top of stack
    fn pop(&mut self) -> Option<Self::Item>;
    /// Return Option<&item:Item> from the top of stack
    fn top(&mut self) -> Option<&Self::Item>;
    /// Return True if stack is empty
    fn is_empty(&self) -> bool;
    /// Return the number of items currently in the stack.
    fn size(&self) -> usize;
}

/// Vector based implementation of StackTrait
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self{
        let stack_items : Vec<T> = vec![];
        Self { items: stack_items }
    }    
}

impl<T> StackTrait for Stack<T> {
    type Item = T;

    fn push(&mut self, item: Self::Item) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
    fn top(&mut self) -> Option<&Self::Item> {
        self.items.last()
    }
    fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
    fn size(&self) -> usize {
        self.items.len()
    }
}
