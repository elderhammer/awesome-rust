pub mod stack {
    // 是个栈，使用Vec作为容器，支持泛型
    #[derive(Debug)]
    pub struct Stack<T> {
        top: usize, // q记录栈顶的索引？yes
        data: Vec<T>
    }

    // 方法有new/push/pop/peek/is_empty/size
    impl<T> Stack<T> {
        fn new() -> Self {
            Stack {
                top: 0,
                data: Vec::new()
            }
        }

        fn push(&mut self, data: T) {
            self.data.push(data);
            self.top += 1
        }

        // q不清楚该用Result还是Option
        fn pop(&mut self) -> Option<T> {
            // error没有考虑空的情况，考虑容器是否为空应该是个常规情况！
            if self.top == 0 { return None; }

            self.top -= 1;
            self.data.pop()
        }

        // q不知道要怎么取vec的元素，并且不持有所有权 a通过get方法
        // q不知道该返回引用还是新的一份拷贝 a考虑到泛型要实现Clone接口，还是返回引用吧
        fn peek(&self) -> Option<&T> {
            /*
            let data = self.data.pop();

            match data {
                Some(data) => {
                    // 因为是泛型，要拷贝就要实现对应的方法，所以引用是个不错的方案
                },
                None => None
            }
            */

            if self.top == 0 { return None; }
            self.data.get(self.top - 1)
        }

        fn is_empty(&self) -> bool {
            self.top == 0
        }

        fn size(&self) -> usize {
            // q这样不会发生move吗？a会，但是因为usize已经实现了拷贝，所以尽管move
            self.top
        }
    }

    /**
     * 括号匹配
     */
    pub fn par_checker(par: &str) -> bool {
        // 解析字符串
        let mut chars:Vec<char> = Vec::new();
        // q将字符串解析成单个字符的方法是什么？a直接调用&str.chars()
        for c in par.chars() { // e&str.chars()返回的是char，而不是&str！q这两者有什么区别？
            chars.push(c);
        }

        // 逐个检查
        let mut balance = true;
        let mut stack = Stack::new();
        /**
         * q在遍历的情况下，move其中的元素会发生什么？
         * qfor到底发生了什么？
         */
        /*
        for i in 0..chars.len() {
            let c = chars[i];

            // 3种情况：左括号、右括号、非括号
            if c == '(' {
                stack.push(c);
            } else if c == ')' {
                if stack.is_empty() {
                    balance = false
                } else {
                    stack.pop();
                }
            }
        }
        */

        let mut index = 0;
        while index < chars.len() && balance { // qwhile到底发生了什么事情？
            let c = chars[index]; // q&str实现了Copy这个trait吗？a要查阅资料

            // 3种情况：左括号、右括号、非括号
            if '(' == c || '{' == c || '[' == c {
                stack.push(c);
            } else if ')' == c || ']' == c || '}' == c {
                if stack.is_empty() {
                    balance = false
                } else {
                    let cc = stack.pop().unwrap();
                    balance = par_match(cc, c)
                }
            }
            index += 1;
        }

        balance && stack.is_empty()
    }

    pub fn par_match(open: char, close: char) -> bool {
        let opens = "({[";
        let closes = ")}]";
        opens.find(open) == closes.find(close)
    }
}