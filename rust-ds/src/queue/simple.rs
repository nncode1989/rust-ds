use super::QueueTrait;

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
    fn enqueue(&mut self, item: Self::Item) {}
    /// take item::Item from the front of queue.
    fn dequeue(&mut self) -> Option<Self::Item> {
        None
    }
}
