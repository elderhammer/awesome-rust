fn main() {
    println!("Hello, world!");

    /**
     * 测试二分查找
     */
    let nums = [1,3,5,7,9];
    let num = 5;
    let res = binary_search(&nums, num);
    println!("在 {:?} 二分查找 {} 的结果：{}", nums, num, res);

    /**
     * 测试哈希表
     */
    let mut hash_map = HashMap::new(11);
    hash_map.insert("foo".to_string(), "bar"); // q.为什么value可以传&str？a."bar"的生命周期是static
    println!("{:?}", hash_map);
    println!("key: {}, value: {}", "foo".to_string(), hash_map.get(String::from("foo")).unwrap());
    hash_map.remove("foo".to_string());
    println!("{:?}, len {}", hash_map, hash_map.len());
}

/**
 * 有3个特别的点：
 * 1.长度从1开始，下标从0开始；
 * 2.会出现low和hight重叠的情况，微积分研究极限，这里数组中只有一个元素就是极限情况；
 * 3.在不等的情况下，判断逻辑可选左开右闭，也可以选左闭右开；
 */
fn binary_search(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut hight = nums.len() - 1; // 要注意长度与下标的关系，下标从0开始，长度从1开始
    let mut found = false;

    while low <= hight && !found { // 可用有3个元素的例子来说明等于的必要性
        // 左移除2，同时避免长度溢出
        let mid: usize = low + ((hight - low) >> 1);

        if num == nums[mid] {
            found = true
        } else if num < nums[mid] {
            hight = mid - 1
        } else {
            low = mid + 1
        }
    }

    found
}

/**
 * Todo
 * 插值查找，类似二分查找，适用于已排序的、均分的数据
 * 思路：利用线性内插法来找中点以及上下限
 */
fn inter_search(nums: &[i32], num: i32) -> bool {
    true
}

/**
 * Todo
 * 指数查找，类似二分查找，适用于已排序的、无边界的数据
 * 思路：通过查看2^n，n=0,1,2,3,...，来确定上下限，然后局部应用二分查找或者插值查找
 */
fn e_search(nums: &[i32], num: i32) -> bool {
    true
}

/**
 * 哈希表要解决的问题：
 * 1.扩容；
 * 
 * 2.哈希函数；
 * 哈希函数计算分两步：一是根据输入集特点设计映射函数，二是用映射得到的值mod哈希表的长度（取余保证范围），余数即为槽的序号。
 * 
 * 3.冲突；
 * 开放地址法v1：如果发生冲突，尝试下一个槽位，直到发现空的槽位。 
 * 开放地址法v2：v1如果发生多次冲突，顺序查找将变得费时。v2的改进点是跳过n个槽，尝试第n+1个槽。要注意的一点是，应用v2时表大小要是素数，原因自行google。
 * 拉链法：对冲突的槽位设置一个链表来保存数据项，在链表的查找可以排序后利用二分查找，如果链过长，则考虑将链改成树，更稳定。
 */
#[derive(Debug, Clone, PartialEq)]
struct HashMap<T> {
    size: usize,
    key: Vec<String>, // key保存键
    value: Vec<T> // value保存数据
}

impl<T: Clone + PartialEq + Default> HashMap<T> {
    fn new(size: usize) -> Self {
        let mut key: Vec<String> = Vec::with_capacity(size);
        let mut value: Vec<T> = Vec::with_capacity(size);
        for _i in 0..size { // 初始化
            key.push("".to_string());
            value.push(Default::default())
        }

        HashMap {
            size: size,
            key: key,
            value: value
        }
    }

    fn hash(&self, key: &str) -> usize {
        let mut sum = 0;
        for (i, c) in key.chars().enumerate() {
            sum += (i + 1) * (c as usize);
        }
        sum % self.size
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.size
    }

    /**
     * 分集中情况：
     * 1.槽无数据就直接插入；
     * 2.有数据就再哈希，找到就插入，如果满了就报错；
     */
    fn insert(&mut self, key: String, value: T) {
        let slot = self.hash(&key);
        if 0 == self.key[slot].len() {
            self.key[slot] = key;
            self.value[slot] = value;
        } else {
            let mut nx_slot = self.rehash(slot);
            while 0 != self.key[nx_slot].len() && key != self.key[nx_slot] {
                nx_slot = self.rehash(nx_slot);
                if nx_slot == slot {
                    println!("Error: map is full!");
                    return;
                }
            }

            self.key[slot] = key;
            self.value[slot] = value;
        }
    }

    fn remove(&mut self, key: String) -> Option<T> {
        // 通过hash找到
        let slot = self.hash(&key);
        if key == self.key[slot] {
            let value = self.value[slot].clone();
            // 清空
            self.key[slot] = "".to_string();
            self.value[slot] = Default::default();
            return Some(value);
        }

        // 通过rehash找到
        let mut nx_slot = self.rehash(slot);
        while key != self.key[nx_slot] && slot != nx_slot {
            nx_slot = self.rehash(nx_slot)
        }
        if key == self.key[nx_slot] {
            let value = self.value[nx_slot].clone();
            // 清空
            self.key[nx_slot] = "".to_string();
            self.value[nx_slot] = Default::default();
            return Some(value);
        }

        None
    }

    fn get(&self, key: String) -> Option<&T> {
        // 通过hash找到
        let slot = self.hash(&key);
        if key == self.key[slot] {
            return self.value.get(slot);
        }

        // 通过rehash找到
        let mut nx_slot = self.rehash(slot);
        while key != self.key[nx_slot] && slot != nx_slot {
            nx_slot = self.rehash(nx_slot)
        }
        if key == self.key[nx_slot] {
            return self.value.get(nx_slot);
        }

        None
    }

    fn contains(&self, key: String) -> bool {
        self.key.contains(&key)
    }

    // 计算槽数据不为0的情况
    fn len(&self) -> usize {
        let mut len = 0;
        for key in self.key.iter() {
            if 0 != key.len() {
                len += 1
            }
        }
        
        len
    }
}