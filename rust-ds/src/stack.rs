#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack_push() {
        let mut st: Stack<i16> = Stack::new(Some(3));
        assert_eq!(st.stack_length(), 0);
        assert_eq!(st.push(1), Some(0));
        assert_eq!(st.push(2), Some(1));
        assert_eq!(st.stack_length(), 2);
        assert_eq!(st.push(3), Some(2));
        assert_eq!(st.push(4), None);
    }
    #[test]
    fn test_stack_pop() {
        let mut st: Stack<i16> = Stack::new(Some(3));
        assert_eq!(st.pop(), None);
        assert_eq!(st.push(1), Some(0));
        assert_eq!(st.push(2), Some(1));
        assert_eq!(st.push(3), Some(2));

        assert_eq!(st.pop(), Some(3));
        assert_eq!(st.pop(), Some(2));
        assert_eq!(st.push(4), Some(1));
        assert_eq!(st.pop(), Some(4));
        assert_eq!(st.pop(), Some(1));
        assert_eq!(st.pop(), None);
        assert_eq!(st.pop(), None);
    }
}

///Implementation of a generic stack
pub struct Stack<T> {
    items: Vec<T>,
    max_size: Option<usize>,
    top_of_stack: Option<usize>,
}
impl<T> Stack<T> {
    pub fn new(size: Option<usize>) -> Self {
        match size {
            None => {
                return Stack {
                    items: Vec::with_capacity(10),
                    max_size: Some(10),
                    top_of_stack: None,
                }
            }
            Some(size) => {
                return Stack {
                    items: Vec::with_capacity(size),
                    max_size: Some(size),
                    top_of_stack: None,
                }
            }
        }
    }
    pub fn push(&mut self, item: T) -> Option<usize> {
        match self.top_of_stack {
            None => {
                self.top_of_stack = Some(0);
            }
            Some(val) => {
                if val < self.max_size.unwrap() - 1 {
                    self.top_of_stack = Some(val + 1);
                } else {
                    return None;
                }
            }
        }
        self.items.push(item);
        return self.top_of_stack;
    }
    pub fn stack_length(&self) -> usize {
        return self.items.len();
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.top_of_stack {
            None => return None,
            Some(0) => {
                self.top_of_stack = None;
            }
            Some(val) => {
                self.top_of_stack = Some(val - 1);
            }
        }
        return self.items.pop();
    }
}
