/// Queue Trait.
pub trait QueueTrait {
    type Item;
    /// Add item::Item to the end of queue.
    fn enqueue(&mut self, item: Self::Item);
    /// take item::Item from the front of queue.
    fn dequeue(&mut self) -> Option<Self::Item>;
}



mod circular;
pub mod simple;
