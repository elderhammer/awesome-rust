pub mod foo {
    /**
     * 用链表来实现一个Vec及其部分接口
     */
    use std::fmt::Debug;

    /**
     * 实现链表结点
     */
    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Link<T>
    }

    type Link<T> = Option<Box<Node<T>>>;

    impl<T> Node<T> {
        fn new(val: T) -> Self {
            Node {
                data: val,
                next: None
            }
        }
    }

    /**
     * 实现LVec，索引从0开始
     */
    #[derive(Debug)]
    pub struct LVec<T> {
        size: usize,
        head: Link<T>
    }

    impl<T: Debug + Copy> LVec<T> {
        pub fn new() -> Self {
            LVec {
                size: 0,
                head: None
            }
        }

        pub fn clear(&mut self) {
            self.size = 0;
            self.head = None;
        }

        pub fn len(&self) -> usize {
            self.size
        }

        pub fn is_empty(&self) -> bool {
            0 == self.size
        }

        /**
         * 新元素要加到末尾，分2种情况：
         * 1.空的
         * 2.非空
         */
        pub fn push(&mut self, item: T) {
            let node = Some(Box::new(Node::new(item)));

            // 空的，直接加到head
            if self.is_empty() {
                self.head = node;
            } else { // 非空的，要找到最后一个结点
                let mut curr = self.head.as_mut().unwrap();
                for _i in 0..self.size - 1 {
                    curr = curr.next.as_mut().unwrap();
                }

                curr.next = node;
            }

            self.size += 1
        }

        /**
         * 弹出最后一个元素，分2种情况：
         * 1.空的
         * 2.非空
         */
        pub fn pop(&mut self) -> Option<T> {
            // 空的，直接返回None
            if self.is_empty() {
                None
            } else if 1 == self.size { // 只有一个元素
                self.head.take().map(|node| {
                    node.data
                })
            } else { // 大于等于2个
                // 找到倒数第2个，take拿到next的所有权并留下空指针，map转换类型
                let mut curr = self.head.as_mut().unwrap(); // 类型为&Box<Node<T>>
                for _i in 0..self.size - 2 { // t注意，0..2表示左闭右开区间，0..=2表示闭区间
                    curr = curr.next.as_mut().unwrap();
                }

                curr.next.take().map(|node| {
                    node.data
                })
            }
        }

        pub fn insert(&mut self, mut idx: usize, item: T) {
            // 限定范围
            if idx >= self.size { idx = self.size; }

            let mut node = Node::new(item);
            if self.is_empty() { // 空的，直接加到head
                self.head = Some(Box::new(node))
            } else if 0 == idx { // 加到第一位
                node.next = self.head.take();
                self.head = Some(Box::new(node))
            } else { // 加到中间或末尾
                let mut curr = self.head.as_mut().unwrap();
                for _i in 0..idx - 1 {
                    curr = curr.next.as_mut().unwrap();
                }

                node.next = curr.next.take();
                curr.next = Some(Box::new(node))
            }

            self.size += 1;
        }

        pub fn remove(&mut self, idx: usize) -> Option<T> {
            // 限定范围
            if idx >= self.size { return None; }

            self.size -= 1;
            if 0 == idx {
                let mut head = self.head.take().unwrap();
                self.head = head.next.take();
                Some(head.data)
            } else {
                // 找到目标元素的上一个元素的指针
                let mut curr = self.head.as_mut().unwrap();
                for _i in 0..idx - 1 {
                    curr = curr.next.as_mut().unwrap();
                }

                let mut target = curr.next.take().unwrap();
                curr.next = target.next.take(); // q.不写take会咋样？a报错从引用中操作move

                Some(target.data)
            }
        }

        pub fn append(&mut self, lvec: &mut Self) {
            while let Some(node) = lvec.head.as_mut().take() {
                self.push(node.data); // 需要拿到node，并且拥有所有权
                lvec.head = node.next.take();
            }
            lvec.clear()
        }

        pub fn print_lvec(&self) {
            let mut curr = self.head.as_ref();
            while let Some(node) = curr {
                println!("lvec val: {:#?}", node.data);
                curr = node.next.as_ref(); 
            }
        }
    }
}