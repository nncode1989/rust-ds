use std::fmt::Display;

use super::QueueTrait;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue_empty() {
        let mut sq: Circular<i16> = Circular::<i16>::new(4);
        assert_eq!(None, sq.dequeue());
    }

    #[test]
    fn test_queue_non_empty() {
        let mut sq: Circular<i16> = Circular::<i16>::new(4);
        assert_eq!(true, sq.enqueue(10));
        assert_eq!(true, sq.enqueue(11));
        assert_eq!(true, sq.enqueue(12));
        // assert_eq!(true,sq.enqueue(13));
        // assert_eq!(false,sq.enqueue(14));
        assert_eq!(Some(10), sq.dequeue());
        assert_eq!(Some(11), sq.dequeue());
        assert_eq!(Some(12), sq.dequeue());
        // assert_eq!(Some(13), sq.dequeue());
        assert_eq!(None, sq.dequeue());
    }

    #[test]
    fn test_queue_random() {
        let mut sq: Circular<i16> = Circular::<i16>::new(4);
        assert_eq!(true, sq.enqueue(10));
        assert_eq!(true, sq.enqueue(11));
        assert_eq!(Some(10), sq.dequeue());
        assert_eq!(true, sq.enqueue(12));
        assert_eq!(Some(11), sq.dequeue());
        assert_eq!(true, sq.enqueue(13));
        assert_eq!(true, sq.enqueue(14));
        assert_eq!(Some(12), sq.dequeue());
        assert_eq!(Some(13), sq.dequeue());
        assert_eq!(Some(14), sq.dequeue());
        assert_eq!(None, sq.dequeue());
        assert_eq!(None, sq.dequeue());
    }
}

struct Circular<T> {
    items: Vec<Option<T>>,
    max_size: usize,
    read: Option<usize>,
    write: Option<usize>,
}

impl<T> Circular<T>
where
    T: Clone,
{
    pub fn new(max_items: usize) -> Self {
        Circular {
            items: vec![],
            max_size: max_items,
            read: None,
            write: None,
        }
    }
}

impl<T> QueueTrait for Circular<T>
where
    T: Clone + Display,
{
    type Item = T;
    // Add item::Item to the next availible slot of queue.
    fn enqueue(&mut self, item: Self::Item) -> bool {
        match self.write {
            None => {
                self.write = Some(0);
                self.read = Some(0);
                self.items.push(Some(item));
                return true;
            }
            Some(wval) => {
                match self.read {
                    None => {
                        if (wval + 1) % self.max_size != 0 {
                            self.write = Some((wval + 1) % self.max_size);
                            self.items.push(Some(item));
                            return true;
                        }
                        false
                    }
                    Some(rval) => {
                        if (wval + 1) % self.max_size == rval {
                            println!("Queue full");
                            return false;
                        } else {
                            self.write = Some((wval + 1) % self.max_size);
                            if self.write.unwrap() > self.items.len() - 1 {
                                self.items.push(Some(item));
                            } else {
                                self.items[self.write.unwrap()] = Some(item);
                            }

                            return true;
                        }
                    }
                };
            }
        }
        return false;
    }
    /// take item::Item from the front of queue.
    fn dequeue(&mut self) -> Option<Self::Item> {
        match self.read {
            None => {
                return None;
            }
            Some(rval) => {
                let i = self.items[rval].clone();
                self.items[rval] = None;
                if rval != self.write.unwrap() {
                    self.read = Some((rval + 1) % self.max_size);
                } else {
                    self.read = None;
                    self.write = None;
                }
                return i;
            }
        }
    }
}
