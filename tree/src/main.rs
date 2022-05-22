fn main() {
    println!("Hello, world!");

    /**
     * 测试二叉树
     */
    let mut btree = BTree::new("root");
    println!("root tree: {:?}", btree);
    println!("key is {}", btree.get_key());
    btree.set_key("bar");
    println!("key is {}", btree.get_key());

    btree.insert_left_tree("left");
    btree.insert_right_tree("right");
    println!("root tree: {:?}", btree);
    println!("left tree: {:?}", btree.get_left());
    println!("right tree: {:?}", btree.get_right());
    println!("root tree: {:?}", btree);

    btree.preorder();
    btree.inorder();
    btree.postorder();

    /**
     * 测试二叉堆
     */
    let mut bh = BHeap::new();
    let nums = [-1,0,2,3,4];
    bh.push(10); bh.push(9);
    bh.push(8); bh.push(7); bh.push(6);
    bh.build_add(&nums);
    println!("empty: {:?}", bh.is_empty());
    println!("min: {:?}", bh.min());
    println!("pop min: {:?}", bh.pop());
    bh.build_new(&nums);
    println!("size: {:?}", bh.len());
    println!("pop min: {:?}", bh.pop());
}

#[derive(Debug, Clone)]
struct BTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>
}

type Link<T> = Option<Box<BTree<T>>>;

impl<T: Clone + std::fmt::Debug> BTree<T> {
    fn new(key: T) -> Self {
        BTree {
            key: key,
            left: None,
            right: None
        }
    }

    fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let mut left = Self::new(key);
            self.left = Some(Box::new(left));
        } else {
            let mut node = Self::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            self.right = Some(Box::new(Self::new(key)));
        } else {
            let mut node = Self::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    // 克隆一个出来
    fn get_left(&self) -> Link<T> {
        self.left.clone() // clone对Some、None都适用
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: T) {
        self.key = key
    }

    // 前序遍历：中左右
    fn preorder(&self) {
        println!("{:?}", &self.key);
        if self.left.is_some() {
            self.left.as_ref().unwrap().preorder();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().preorder();
        }
    }

    // 后序遍历：左右中
    fn postorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().postorder();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().postorder();
        }
        println!("{:?}", &self.key);
    }

    // 中序遍历：左中右
    fn inorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().inorder();
        }
        println!("{:?}", &self.key);
        if self.right.is_some() {
            self.right.as_ref().unwrap().inorder();
        }
    }
}

/**
 * 二叉堆
 * 
 * 特点是用数组等线性数据结构来保存，父子之间下标关系可通过计算得到。
 * 若父节点下标为n，则左子结点下标为n<<1，右子结点为(n<<1)+1。
 * 若子节点下标为n，则父节点下标为n>>1。
 */
macro_rules! parent {
    ($child:ident) => {
        $child >> 1
    };
}

macro_rules! left_child {
    ($parent:ident) => {
        $parent << 1
    };
}

macro_rules! right_child {
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}

#[derive(Debug, Clone)]
struct BHeap {
    size: usize,
    data: Vec<i32>
}

impl BHeap {
    fn new() -> Self {
        BHeap { size: 0, data: vec![0]} // 注意，第一个元素是占位符
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    /**
     * 按照定义，第一个元素最小
     */
    fn min(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[1])
        }
    }

    /**
     * 新加入的元素可能会破坏原来的有序关系，所以要向上移动维持平衡
     */
    fn push(&mut self, item: i32) {
        self.data.push(item);
        self.size += 1;
        self.move_up(self.size)
    }

    /**
     * 晋升
     * 举例来说，在小顶堆中，新元素与父元素比较，如果小于父元素，则交换位置，重复直到碰到更小的父节点
     */
    fn move_up(&mut self, mut child: usize) {
        let mut parent = parent!(child);
        while parent > 0 {
            if self.data[child] < self.data[parent] {
                self.data.swap(child, parent);
                child = parent;
                parent = parent!(child);
            } else {
                break
            }
        }
    }

    /**
     * 分3种情况：
     * 1.没有结点
     * 2.只有一个结点
     * 3.有多个结点
     */
    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else if 1 == self.size {
            self.size -= 1;
            self.data.pop()
        } else {
            self.data.swap(1, self.size);
            let val = self.data.pop();
            self.size -= 1;
            // 此时，根节点不是最小的，要调整
            self.move_down(1);
            val
        }
    }

    /**
     * 降职
     * 思路和晋升差不多，最大的区别就是要从左右孩子中挑出小的那个
     */
    fn move_down(&mut self, mut parent: usize) {
        // 主题是个循环逻辑，不断往下沉
        loop {
            // 先判断是否为叶子节点
            let lc = left_child!(parent);
            if lc > self.size { break; }

            // 和最小的孩子比较，如果当前结点下，则交换位置
            let mc = self.min_child(parent);
            if self.data[mc] < self.data[parent] {
                self.data.swap(mc, parent);
            }
            parent = mc
        }
    }

    fn min_child(&self, parent: usize) -> usize {
        let lc = left_child!(parent);
        let rc = right_child!(parent);
        if rc > self.size {
            lc
        } else if self.data[rc] > self.data[lc] {
            lc
        } else {
            rc
        }
    }

    fn build_new(&mut self, arr: &[i32]) {
        for _i in 1..=self.size {
            self.data.pop();
        }

        for &i in arr {
            self.data.push(i);
        }
        self.size = arr.len();

        // 调整堆
        let len = self.size;
        let mut p = parent!(len);
        while p > 0 {
            self.move_down(p);
            p -= 1;
        }
    }

    fn build_add(&mut self, arr: &[i32]) {
        for &i in arr {
            self.data.push(i);
        }
    }
}

// /**
//  * Todo：Box换成Rc
//  * 读取以空格间隔的表达式
//  */
// fn read_exp(exp: &str) -> BTree<&str> {
//     let ops: Vec<&str> = vec!["+","-","/","*"];
//     let mut parent_stack: Vec<BTree<&str>> = Vec::new();
//     let mut curr = BTree::new("");
//     for c in exp.split_whitespace() {
//         if "(" == c {
//             let mut left = BTree::new("");
//             curr.left = Some(Box::new(left));
//             parent_stack.push(curr);
//             curr = left;
//         } else if "0" <= c && c <= "9" {
//             curr.set_key(c);
//             curr = parent_stack.pop().unwrap();
//         } else if ops.contains(&c) {
//             curr = parent_stack.pop().unwrap();
//             curr.set_key(c);
//             let mut right = BTree::new("");
//             curr.right = Some(Box::new(right));
//             parent_stack.push(curr);
//             curr = right;
//         } else {
//             curr = parent_stack.pop().unwrap();
//         }
//     }
// }