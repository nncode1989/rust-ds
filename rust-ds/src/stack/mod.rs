#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_int() {
        let mut stack: Stack<i8> = Stack::new();
        let stack_mut = &mut stack;

        assert_eq!(stack_mut.pop(), None);

        stack_mut.push(10);

        assert_eq!(stack_mut.pop(), Some(10));
        assert_eq!(stack_mut.pop(), None);
        assert_eq!(stack_mut.pop(), None);
        assert_eq!(stack_mut.push(10), Some(1));
        assert_eq!(stack_mut.push(20), Some(2));
        assert_eq!(stack_mut.push(30), Some(3));
        assert_eq!(stack_mut.pop(), Some(30));
        assert_eq!(stack_mut.pop(), Some(20));
        assert_eq!(stack_mut.pop(), Some(10));
        assert_eq!(stack_mut.pop(), None);
        assert_eq!(stack_mut.push(101), Some(1));
    }
    #[test]
    fn test_stack_float() {
        let mut stack: Stack<f32> = Stack::new();
        let stack_mut = &mut stack;

        assert_eq!(stack_mut.pop(), None);

        stack_mut.push(10.2);

        assert_eq!(stack_mut.pop(), Some(10.2));
    }

    #[test]
    fn test_stack_string() {
        let mut stack: Stack<String> = Stack::new();
        let stack_mut = &mut stack;

        assert_eq!(stack_mut.pop(), None);

        let s = String::from("Test");
        stack_mut.push(s);
        assert_eq!(stack_mut.pop(), Some(String::from("Test")));
    }
}
///A Generic Struct that represents a Stack DataStructure
/// The DS , owns athe elements that are pushed to the stack.
pub struct Stack<T> {
    items: Vec<T>,
    top_of_stack: Option<u16>,
}
///Implemention of Stack Operations
impl<T> Stack<T> {
    pub fn new() -> Self {
        return Stack {
            items: vec![],
            top_of_stack: None,
        };
    }
    /// PUSH : Add an item T to the top of Stack
    pub fn push(&mut self, value: T) -> Option<u16> {
        self.items.push(value);
        if let None = self.top_of_stack {
            self.top_of_stack = Some(1);
        } else {
            self.top_of_stack = Some(self.top_of_stack.unwrap() + 1)
        }
        println!("tos: {}", self.top_of_stack.unwrap());
        return self.top_of_stack;
    }
    /// POP  : Remove an Item T from the top of Stack
    pub fn pop(&mut self) -> Option<T> {
        if let None = self.top_of_stack {
            return None;
        } else {
            if self.top_of_stack.unwrap() == 1 {
                self.top_of_stack = None;
            } else {
                self.top_of_stack = Some(self.top_of_stack.unwrap() - 1);
            }
        }
        return self.items.pop();
    }
}
