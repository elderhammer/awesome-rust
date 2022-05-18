use queue::queue::Queue;

fn main() {
    println!("Hello, world!");

    let mut queue = Queue::new(10);
    queue.enqueue(1);
    println!("{:?}", queue);

    let mut v = vec![1,2,3];
    v.insert(1, 3);
    assert_eq!(v, [1,3,3]);
}
