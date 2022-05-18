fn main() {
    println!("Hello, world!");

    /**
     * 测试队列
    let mut queue = queue::foo::Queue::new(10);
    queue.enqueue(1);
    println!("{:?}", queue);

    let mut v = vec![1,2,3];
    v.insert(1, 3);
    assert_eq!(v, [1,3,3]);
     */

    let names = vec!["jaky", "william", "sala", "luna", "brian"];
    let last_name = queue::foo::hot_potato(names, 8);
    println!("last name is {}", last_name);
}
