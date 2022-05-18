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

    /**
     * 进制转换
     */
    /**
     * e返回类型不对
    pub fn divide_by_two(input: mut u32) -> &str {
     *
     * e可变入参是mut在参数名之前
    pub fn divide_by_two(input: mut u32) -> String {
     */
    pub fn divide_by_two(mut input: u32) -> String {
        // 初始化栈
        let mut stack: Stack<u32> = Stack::new();

        // 迭代取模计算余数，入栈。q中断条件是什么？a因为input是整数类型，所以最后会得到小数并被转换为0
        while input > 0 {
            let diff = input % 2;
            stack.push(diff);
            input /= 2;
        }

        // 出栈输出二进制
        let mut binary = "".to_string(); // q不知道字符串拼接该用String还是&str a因为不知道长度，所以用String
        while !stack.is_empty() { // q还是那个问题，while的时候，删减容器元素会怎样？a容器是同一个
            /**
             * e没有注意返回类型
            let data = stack.pop();
             */
            let data = stack.pop().unwrap().to_string();
            binary += &data; // q对String类型取引用结果是&str？a在defer trait提到，&String需要的时候会自动解引用成&str
        }

        binary
    }

    pub fn base_converter(mut input: u32, base: u32) -> String {
        // t注意，默认索引值的类型为usize
        let digits = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f']; // t元素类型为char
        let digits = ["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"]; // t元素类型为&str

        let mut stack: Stack<u32> = Stack::new();

        // 迭代取模
        while input > 0 {
            let data = input % base; // q取模运算的返回类型是什么？a返回u32
            stack.push(data);
            input /= base;
        }

        // 转换输出
        let mut output = "".to_string();
        while !stack.is_empty() {
            let data = stack.pop().unwrap() as usize; // t这种用法头一次见
            output += &digits[data].to_string(); // q为什么digits中的元素类型为char而不是&str？a看digits的注释
        }

        output
    }

    /**
     * 中缀转后缀
     */
    pub fn infix_to_postfix(infix: &str) -> Option<String> {
        // 括号匹配检查
        if !par_checker(infix) { return None; }

        // 操作符优先级
        let mut op_property = std::collections::HashMap::new();
        /**
         * e注意，单引号字符串类型是char，双引号字符串类型是&str
        op_property.insert('(', 1);op_property.insert(')', 1);
        op_property.insert('+', 2);op_property.insert('-', 2);
        op_property.insert('*', 3);op_property.insert('/', 3);
         */
        op_property.insert("(", 1);op_property.insert(")", 1);
        op_property.insert("+", 2);op_property.insert("-", 2);
        op_property.insert("*", 3);op_property.insert("/", 3);

        let mut op_stack = Stack::new();
        let mut postfix = Vec::new();
        // 从左到右处理中缀表达式
        for token in infix.split_whitespace() { // q不清楚&str有没有按空白进行分隔的方法，也不知道返回值类型是什么 a有方法，叫split_whitespace，返回值类型是&str
            // 如果是0-9或A-Z，则保持顺序，先进先出入队
            if ("0" <= token && token <= "9") || ("A" <= token && token <= "Z") { // q不清楚如何进行范围比较 q2这里的范围比较原理是什么
                postfix.push(token);
            }
            // 如果是左括号，则压栈（隐含存在更高优先级的操作符）
            else if "(" == token {
                op_stack.push(token);
            }
            // 如果是右括号，则最近操作符出栈入队
            else if ")" == token {
                let top = op_stack.pop().unwrap();
                // 如果括号内存在操作符，则该操作符是当前优先级最高的操作符
                if "(" != top {
                    postfix.push(top);
                    op_stack.pop();
                }
            }
            // 接下来考虑没有括号来指示优先级的情况：如果栈内已有操作符，按照分析，要按照优先级越高约接近栈顶进行调整
            else {
                while !op_stack.is_empty() && (
                    op_property[op_stack.peek().unwrap()] >= op_property[token] // q不清楚&str有没有to_char这个方法 a没有
                ) {
                    postfix.push(op_stack.pop().unwrap());
                }
                
                op_stack.push(token);
            }
        }

        // 处理剩下的操作符
        while !op_stack.is_empty() {
            postfix.push(op_stack.pop().unwrap());
        }

        // 生成后缀表达式
        let mut fix = "".to_string();
        /**
         * 
        while !postfix.is_empty() {
            // q有没有快捷办法生成字符串？a暂时没有
            fix += postfix.pop().unwrap();
            fix += &" ".to_string();
        }
        */
        for token in postfix {
            fix += token;
            fix += &" ".to_string();
        }

        Some(fix)
    }

    pub fn postfix_eval(postfix: &str) -> Option<i32> {
        // 包含空格，一个合法的表达式必须有5个字符
        if postfix.len() < 5 { return None; }

        // 从左到右，计算
        let mut op_stack = Stack::new();
        for token in postfix.split_whitespace() {
            if ("0" <= token && token <= "9") || ("A" <= token && token <= "Z") {
                op_stack.push(token.parse::<i32>().unwrap()); // q不知道该怎么把&str转为i32类型 a&str有一个parse的方法可以用，返回结果是个Result
            } else {
                let op1 = op_stack.pop().unwrap();
                let op2 = op_stack.pop().unwrap();
                let op = do_cal(op1, op2, token);
                op_stack.push(op);
            }
        }

        Some(op_stack.pop().unwrap())
    }

    pub fn do_cal(op1: i32, op2: i32, op: &str) -> i32 {
        match op {
            "+" => op1 + op2,
            "-" => op1 - op2, // q为什么不考虑位置？
            "*" => op1 * op2,
            "/" => {
                if op2 == 0 {
                    // q不知道要怎么报错 a使用panic!这个宏
                    panic!("Error: divide by zero");
                }
                op1 / op2
            },
            _ => panic!("Error: unkown operation") // e忘了默认情况
        }
    }
}