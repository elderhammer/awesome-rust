use stack::stack;

fn main() {
    println!("Hello, world!");

    /*
    // 调用例子：初始化，然后分别打印栈顶元素、去除并打印栈顶元素
    let mut stack: Stack<usize> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // q格式是什么？a是{:?}
    // q为什么要unwrap？因为是个Option枚举，值被包含在Some里面
    println!("{:?}, size: {}", stack.peek().unwrap(), stack.size()); // q为什么会报错同时存在可变和不可变的引用？a因为作为入参，“同时“出现在调用函数内部！
    println!("{:?}, size: {}", stack.pop().unwrap(), stack.size());
    */

    let par1 = "(d(){}sd)";
    println!("par {} is {:?}", par1, stack::par_checker(par1));
}
