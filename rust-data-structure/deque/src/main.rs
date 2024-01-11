fn main() {
    println!("Hello, world!");

    /**
     * 测试双端列表
    let mut deque = deque::foo::Deque::new(10);
    deque.add_front(1);
    deque.add_rear(2);
    println!("{:?}", deque);
    let front = deque.remove_front().unwrap();
    let rear = deque.remove_rear().unwrap();
    println!("front is {}, rear is {}", front, rear);
     */

    let pal = "adidida";
    println!("{}", deque::foo::pal_check(pal));
}
