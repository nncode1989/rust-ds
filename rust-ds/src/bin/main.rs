use dslib::cqueue::CQueue;


#[derive(Clone)]
struct Test(i32);

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}

fn main() {
    let mut cq: CQueue<Test> = CQueue::new(3);
        assert_eq!(cq.enqueue(Test(1)), Some(0));
        println!("hii");
        let cval = cq.dequeue();
        println!("hello");
}