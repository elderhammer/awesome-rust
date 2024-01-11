pub mod foo {
    #[derive(Debug)]
    pub struct Queue<T> {
        cap: usize,
        data: Vec<T>
    }

    impl<T> Queue<T> {
        pub fn new(cap: usize) -> Self {
            Queue {
                cap: cap,
                data: Vec::with_capacity(cap)
            }
        }

        pub fn enqueue(&mut self, item: T) -> Result<(), String> { // q不熟悉Result的使用 a第一个指OK的类型，第二个指Err的类型
            // 先判断容量，再添加
            if Self::size(&self) >= self.cap { // q不熟悉内部调用的方式 a采用::
                return Err("No space available".to_string()); // q不熟悉Result的Err a类似枚举，可包含错误信息
            }
            self.data.insert(0, item); // q不熟悉vec的接口 a先将目标位置及其右侧的元素向右移动一个位置，然后插入新元素

            Ok(())
        }

        pub fn dequeue(&mut self) -> Option<T> {
            if Self::size(&self) > 0 {
                self.data.pop()
            } else {
                None
            }
        }

        pub fn is_empty(&self) -> bool {
            0 == Self::size(&self)
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }
    }

    /**
     * 烫手山芋
     */
    pub fn hot_potato(names: Vec<&str>, num: i32) -> &str {
        // 初始化队列
        let mut queue = Queue::new(names.len());
        for name in names {
            queue.enqueue(name);
        }

        // 游戏
        while queue.size() > 1 {
            for i in 1..num {
                let name = queue.dequeue().unwrap();
                queue.enqueue(name);
            }

            // 手上有山芋的人淘汰
            queue.dequeue();
        }

        // 最后一个人
        queue.dequeue().unwrap()
    }
}