use super::QueueTrait;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue_empty() {
        let mut sq: Simple<i16> = Simple::<i16>::new();
        assert_eq!(None, sq.dequeue());
    }

    #[test]
    fn test_queue_non_empty() {
        let mut sq: Simple<i16> = Simple::<i16>::new();
        sq.enqueue(10);
        sq.enqueue(11);
        sq.enqueue(12);
        assert_eq!(Some(10), sq.dequeue());
        assert_eq!(Some(11), sq.dequeue());
        assert_eq!(Some(12), sq.dequeue());
        assert_eq!(None, sq.dequeue());
    }
}

/// Vector based implementation of QueueTrait
pub struct Simple<T> {
    items: Vec<T>,
}

impl<T> Simple<T> {
    pub fn new() -> Self {
        let q_items: Vec<T> = vec![];
        Simple { items: q_items }
    }
}

impl<T> QueueTrait for Simple<T> {
    type Item = T;
    /// Add item::Item to the end of queue.
    fn enqueue(&mut self, item: Self::Item) -> bool{
        self.items.push(item);
        true
    }
    /// take item::Item from the front of queue.
    fn dequeue(&mut self) -> Option<Self::Item> {
        if self.items.len() == 0{
            return None;
        }
        Some(self.items.remove(0))
    }
}
