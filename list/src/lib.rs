pub mod foo {
    // Node Pointer
    /**
     * e注意是Option<Box<Node<T>>>
    type NP<T> = Box<Option<Node<T>>>;
     */
    type NP<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T> {
        data: T,
        next: NP<T> // q不熟悉怎么起别名 a用type
    }

    #[derive(Debug)]
    pub struct List<T> {
        size: usize,
        // head: Box::new(None) // q这里为什么要用NP？aNone表示空指针
        head: NP<T>
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                size: 0,
                head: None
            }
        }

        pub fn is_empty(&self) -> bool {
            0 == Self::size(&self)
        }

        pub fn size(&self) -> usize {
            self.size
        }

        // 新节点总是加到头部
        pub fn push(&mut self, item: T) {
            let node = Box::new(Node {
                data: item,
                /**
                 * q总是加到头部要怎么设置 a画图显示新节点next指向原head节点，head的类型是NP<T>，要利用Option的take取出Box指针（如果存在的话）
                 * q不熟悉take()的用法 a如果是Some(val)，则返回Some(val)，并赋值None。入参是可变引用
                 */
                next: self.head.take()
            });

            self.head = Some(node);
            self.size += 1;
        }

        // 返回链表头结点
        pub fn pop(&mut self) -> Option<T> {
            /**
            // 初级写法
            let node = self.head;
            match node {
                Some(_node) => {
                    self.head = _node.next;
                    self.size -= 1;
                    Some(_node.data)
                },
                None => None
            }
            */

            // 高级写法
            self.head.take().map(|node| {
                self.head = node.next;
                self.size -= 1;
                node.data
            })
        }

        // 获取头结点的引用
        pub fn peek(&self) -> Option<&T> { // q不知道该返回什么 a如果习惯用引用适配器的话，就可以接受Option<&T>是常见情况
            /**
            // 初级写法
            let head = self.head;
            match head {
                Some(_head) => {
                    Some(&_head.data)
                },
                None => None
            }
            */

            // 高级写法
            /**
             * 1. q.as_ref()入参和返回值分别是什么类型？a入参是&self（&Option<T>），返回值是Option<&T>
             * 2. q.map()的常用套路有哪些？a对Option来讲，map很常用，作用是将Option<T>转换为Option<U>。要注意，入参类型是self，所以会发生move。另一个入参是一个闭包函数。如果入参是None，则不变。
             */
            self.head.as_ref().map(|node| &node.data )
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            /**
            // 初级写法
            let node = self.head;
            match node {
                Some(_node) => { // q不了解引用在结构体中的传递规则，例如取结构体的可变引用，那访问其属性时是什么类型？a.访问过程会自动解引用，所以是属性的声明类型
                    Some(&mut _node.data)
                },
                None => None
            }
            */

            // 高级写法
            self.head.as_mut().map(|node| &mut node.data ) // q没见过as_mut方法 a将&mut Option<T>转换为Option<&mut T>
        }

        // into_iter: 链表改变，成为迭代器。意思就是把链表转换为一个迭代器
        pub fn into_iter(self) -> IntoIter<T> {
            // q.不清楚如何实例化一个没有属性名的结构体 a.类似声明，直接当做函数来用，将入参传入即可
            IntoIter(self)
        }

        /**
         * iter: 链表不变，只得到不可变迭代器
         * 注意，在链表里，NP<T>是为了解决递归定义的问题。对外部调用者来说，他们只关心Node类型，所以要返回&Node。
         */
        pub fn iter(&self) -> Iter<T> {
            Iter {
                next: self.head.as_deref() // t.这里有三个动作：一从Option中取出值，二对Box解引用拿到Node<T>，三取Node<T>的引用
            }
        }

        // iter_mut: 链表不变，得到可变迭代器
        pub fn iter_mut(&mut self) -> IterMut<T> {
            IterMut {
                next: self.head.as_deref_mut()
            }
        }
    }

    pub struct IntoIter<T>(List<T>); // t这种叫元组结构体，元组可以有人n>1个元素
    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    pub struct Iter<'a, T: 'a> { // q.为什么生命周期标注要放在T上？a.如果不关心T，可不可以理解为把Iter的生命周期和构造时传入的&Node<T>生命周期保持一致
        next: Option<&'a Node<T>> // q.迭代器要迭代Node<T>还是T？
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T; // t.再次将&T的生命周期和当前结构体关联起来
        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.data
            })
        }
    }

    pub struct IterMut<'a, T: 'a> {
        next: Option<&'a mut Node<T>>
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            /**
             * 首先，发生move是因为Option<&mut Node<T>>没有实现Copy trait，self.next的类型就是这个类型。
             */
            self.next.take().map(|node| { // q.这里为什么要take，Iter的实现为什么不用？a.因为按照引用规则，可变引用只能有一个
                self.next = node.next.as_deref_mut();
                &mut node.data
            })
        }
    }

    // 为链表实现自定义Drop
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            // 首先，思路是按照next指针，逐个释放
            while let Some(head) = self.head.take() {
                // 如果存在next，接着处理
                self.head = head.next;
            }
        }
    }
}