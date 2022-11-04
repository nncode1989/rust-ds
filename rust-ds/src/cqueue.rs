#[cfg(test)]
mod tests {
    use super::CQueue;

    // #[test]
    fn test_enqueue() {
        let mut cq: CQueue<i16> = CQueue::new(3);
        assert_eq!(cq.enqueue(1), Some(0));
        assert_eq!(cq.enqueue(2), Some(1));
        assert_eq!(cq.enqueue(3), Some(2));
        assert_eq!(cq.enqueue(4), None);
    }

    // #[test]
    fn test_dequeue() {
        let mut cq: CQueue<i16> = CQueue::new(3);
        assert_eq!(cq.enqueue(1), Some(0));
        assert_eq!(cq.enqueue(2), Some(1));
        assert_eq!(cq.enqueue(3), Some(2));
        assert_eq!(cq.enqueue(4), None);
        // 1<2<3
        assert_eq!(cq.dequeue(), Some(1));
        //2<3
        assert_eq!(cq.dequeue(), Some(2));
        // //3
        assert_eq!(cq.dequeue(), Some(3));
        // //
        assert_eq!(cq.dequeue(), None);
        // //
        assert_eq!(cq.dequeue(), None);
        // //4
        assert_eq!(cq.enqueue(4), Some(0));
        // //4<5
        assert_eq!(cq.enqueue(5), Some(1));
        assert_eq!(cq.enqueue(6), Some(2));
        assert_eq!(cq.enqueue(5), None);
        assert_eq!(cq.enqueue(5), None);
        assert_eq!(cq.dequeue(), Some(4));
        assert_eq!(cq.dequeue(), Some(5));
        assert_eq!(cq.dequeue(), Some(6));
    }
}

pub struct CQueue<T> {
    items: Vec<Option<T>>,
    max_size: usize,
    insert_pos: Option<usize>,
    read_pos: Option<usize>,
}

impl<T> CQueue<T>
where
    T: Clone,
{
    pub fn new(max_size: usize) -> Self {
        CQueue {
            items: Vec::with_capacity(max_size),
            max_size: max_size,
            insert_pos: None,
            read_pos: None,
        }
    }
    pub fn enqueue(&mut self, item: T) -> Option<usize> {
        match (self.insert_pos, self.read_pos) {
            //First time insert
            (None, None) => {
                self.items.push(Some(item));
                self.insert_pos = Some(0);
                return Some(0);
            }
            //insert and not read so far
            (Some(ip), None) => {
                if (ip + 1) % self.max_size == 0 {
                    //Queue is full
                    return None;
                } else {
                    self.insert_pos = Some((ip + 1) % self.max_size);
                    self.items.push(Some(item));
                    return Some(self.insert_pos.unwrap());
                }
            }
            (Some(ip), Some(rp)) => {
                if (ip + 1) % self.max_size == rp {
                    //Queue is full
                    return None;
                } else {
                    self.insert_pos = Some((ip + 1) % self.max_size);
                    self.items[self.insert_pos.unwrap()] = Some(item);
                    return Some(self.insert_pos.unwrap());
                }
            }
            _other => {
                panic!("invalid state");
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match (self.read_pos, self.insert_pos) {
            (None, None) => {
                //que is empty
                return None;
            }
            (None, Some(_)) => {
                self.read_pos = Some(0);
                let item = self.items.get(0).unwrap().clone();
                self.items[0] = None;
                return item;
            }
            (Some(rp), Some(ip)) => {
                let x = (rp + 1) % self.max_size > ip;
                if (rp + 1) % self.max_size > ip {
                    //que is empty
                    return None;
                } else {
                    self.read_pos = Some((rp + 1) % self.max_size);
                    let item = self.items.get(self.read_pos.unwrap()).unwrap().clone();
                    self.items[self.read_pos.unwrap()] = None;
                    return item;
                }
            }
            _other => {
                panic!("invalid state");
            }
        }
    }
}
