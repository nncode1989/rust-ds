use dslib::{self, queue::QueueTrait, stack::StackTrait};

fn main() {
    let mut sq: dslib::Simple<i16> = dslib::Simple::<i16>::new();
    sq.enqueue(1);
    print!("{}", sq.dequeue().unwrap_or_else(|| -1));
}
