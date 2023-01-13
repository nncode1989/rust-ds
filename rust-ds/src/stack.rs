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

    fn push(&mut self, i: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn top(&mut self) -> Option<&Self::Item>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

/// Vector based implementation of StackTrait
pub struct Stack<T> {
    items: Vec<T>,
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
