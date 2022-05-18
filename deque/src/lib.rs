pub mod foo {
    #[derive(Debug)]
    pub struct Deque<T> {
        cap: usize,
        data: Vec<T>
    }

    impl<T> Deque<T> {
        pub fn new(cap: usize) -> Self {
            Deque {
                cap: cap,
                data: Vec::with_capacity(cap)
            }
        }

        pub fn add_front(&mut self, item: T) -> Result<(), String> {
            // 检查容量
            if Self::size(&self) >= self.cap {
                return Err("no more space".to_string());
            }
            
            self.data.insert(0, item); // qVec的索引是从0开始么？

            Ok(())
        }

        pub fn add_rear(&mut self, item: T) -> Result<(), String> {
            // 检查容量
            if Self::size(&self) >= self.cap {
                return Err("no more space".to_string());
            }

            self.data.push(item);

            Ok(())
        }

        pub fn remove_front(&mut self) -> Option<T> {
            // 检查是否为空
            if Self::size(&self) == 0 { return None; }

            Some(self.data.remove(0))
        }

        pub fn remove_rear(&mut self) -> Option<T> {
            // 检查是否为空
            if Self::size(&self) == 0 { return None; }

            self.data.pop()
        }

        pub fn is_empty(&self) -> bool {
            0 == self.data.len()
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }
    }

    /**
     * 回文检测
     */
    pub fn pal_check(pal: &str) -> bool {
        let mut d = Deque::new(pal.len());
        for b in pal.as_bytes() {
            d.add_rear(b);
        }

        let mut is_pal = true;
        while d.size() > 1 && is_pal {
            let front = d.remove_front().unwrap();
            let rear = d.remove_rear().unwrap();
            if front != rear {
                is_pal = false
            }
        }

        is_pal
    }
}