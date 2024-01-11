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

    /**
     * 测试BST
     */
    let mut bst: BSTree<i32, &str> = BSTree::new();
    bst.insert(2, "foo");
    bst.insert(1, "bar");
    bst.insert(3, "far");
    println!("bst: {:?}", bst);
    println!("empty: {}", bst.is_empty());
    println!("len: {}", bst.len());
    println!("get: {}", *(bst.get(1).unwrap()));
    println!("bst: {:?}", bst);
    println!("search: {}", bst.search(4));
    let (key, val) = bst.min();
    println!("min: {} -> {}", *(key.unwrap()), *(val.unwrap()));
}

/**
 * 二叉树
 */
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

/**
 * BST
 */
#[derive(Debug)]
struct BSTree<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Pointer<T, U>,
    right: Pointer<T, U>
}

type Pointer<T, U> = Option<Box<BSTree<T, U>>>;

impl<T, U> BSTree<T, U>
where // 老是以为where后面要跟冒号
    T: Clone + std::fmt::Debug + Ord, // key要能支持比较
    U: Clone + std::fmt::Debug
{
    fn new() -> Self {
        BSTree {
            key: None,
            val: None,
            left: None,
            right: None
        }
    }

    // 判断是否为空树
    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    // q.什么是树的长度？a.结点数
    fn len(&self) -> u32 {
        self.cal_node()
    }

    // 将调用者当做根节点，递归记算
    fn cal_node(&self) -> u32 {
        if self.is_empty() {
            return 0;
        }

        let mut len = 1;

        if self.has_left_child() {
            len += self.left.as_ref().unwrap().cal_node();
        }

        if self.has_right_child() {
            len += self.right.as_ref().unwrap().cal_node();
        }

        len
    }

    // fn preorder()
    // fn inorder()
    // fn postorder()

    // 插入新元素
    fn insert(&mut self, key: T, val: U) {
        if self.is_empty() { // 空树直接插入
            self.key = Some(key);
            self.val = Some(val);
        } else if *(self.key.as_ref().unwrap()) == key { // 如果是当前结点，则更新val
            // q.这个比较要怎么写才优雅？
            self.val = Some(val);
        } else { // 找到合适的子树，递归insert
            let curr_key = self.key.as_ref().unwrap();
            if *curr_key > key {
                // 递归左子树
                if self.has_left_child() {
                    self.left.as_mut().unwrap().insert(key, val);
                } else { // 不存在的话，就要创建一个
                    let mut left = BSTree::new();
                    left.insert(key, val);
                    self.left = Some(Box::new(left));
                }
            } else {
                // 递归右子树
                if self.has_right_child() {
                    self.right.as_mut().unwrap().insert(key, val);
                } else { // 不存在的话，就要创建一个
                    let mut right = BSTree::new();
                    right.insert(key, val);
                    self.right = Some(Box::new(right));
                }
            }
        }
    }

    // 获取结点的value
    fn get(&self, key: T) -> Option<&U> {
        if self.is_empty() {
            None
        } else if *(self.key.as_ref().unwrap()) == key {
            Some(self.val.as_ref().unwrap())
        } else {
            let curr_key = self.key.as_ref().unwrap();
            if *curr_key > key {
                // 递归左子树
                if self.has_left_child() {
                    self.left.as_ref().unwrap().get(key)
                } else {
                    None
                }
            } else {
                // 递归右子树
                if self.has_right_child() {
                    self.right.as_ref().unwrap().get(key)
                } else {
                    None
                }
            }
        }
    }

    fn search(&self, key: T) -> bool {
        self.get(key).is_some()
    }

    /**
     * Todo：利用Rc来实现存储父结点指针
     */
    // fn remove(&mut self, key: T) -> Option<U> {
    //     if self.is_empty() {
    //         None
    //     } else if *(self.key.as_ref().unwrap()) == key {
    //         // 清理数据
    //         self.key.take();
    //         let val = self.val.take();

    //         if self.has_left_child() || self.has_right_child() {
    //             let mut change: &mut BSTree<T, U>;
    //             if self.has_right_child() {
    //                 // 有两个子节点，则右子树任意key必然大于左子树任意key，所以从右子树中找到最小结点，替换当前结点即可
    //                 let mut right = self.right.as_mut().unwrap();
    //                 change = right.min_node().unwrap();
    //             } else {
    //                 change = self.left.as_deref_mut().unwrap();
    //             }
    
    //             // 转移
    //             self.key = change.key.take();
    //             self.val = change.val.take();
    //             self.left = change.left.take();
    //             self.right = change.right.take();
    //         }

    //         Some(val.unwrap())
    //     } else {
    //         None
    //     }
    // }

    fn has_left_child(&self) -> bool {
        self.left.is_some() && !self.left.as_ref().unwrap().is_empty()
    }

    fn has_right_child(&self) -> bool {
        self.right.is_some() && !self.right.as_ref().unwrap().is_empty()
    }

    // 最小值在最左侧
    fn min_node(&mut self) -> Option<&mut BSTree<T, U>> {
        if self.has_left_child() {
            self.left.as_mut().unwrap().min_node()
        } else if !self.is_empty() {
            Some(self)
        } else {
            None
        }
    }

    // 最小值在最左侧
    fn min(&self) -> (Option<&T>, Option<&U>) {
        match &self.left {
            Some(node) => node.min(),
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            }
        }
    }

    // 找到最大的kv、最小的kv
    // fn max()
    // fn min()

    // 如何迭代一棵树?
    // fn iter()
}