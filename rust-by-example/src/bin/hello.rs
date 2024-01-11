const FOO: i32 = 32;

// https://doc.rust-lang.org/stable/rust-by-example/hello.html
fn main() {
    println!("Hello world!");

    // 占位符{}
    println!("{} days", 31);

    // 可以根据位置复用
    println!("{0}, this is {1}. {1}, this is {0}", "张三", "李四");

    // 直接在占位符里面指定变量名，还不用关心变量实际的顺序
    println!("{a} {b} {c}", c="c", b="b", a="a");

    // 进制转换很方便，写法就跟类型声明一样
    println!("base 10: {}", 123);
    println!("base 2: {:b}", 123);
    println!("base 8: {:o}", 123);
    println!("base 16: {:x}", 123);
    println!("base 16: {:X}", 123);

    // 左右对齐，当作类型声明一样处理，对其方向跟直接一样，> 表示右对齐，< 表示左对齐
    // 既然要对齐，那肯定是出现空格了，所以还要指明打印长度
    println!("{number:>5}", number=1);
    println!("{number:>5}", number=2);
    println!("{number:<5}", number=1);
    println!("{number:<5}", number=2);

    // 可以在对齐箭头前声明要填充的字符
    println!("{number:0>5}", number=1);
    println!("{number:🌟>5}", number=1);
    println!("{number:🌟<5}", number=1);

    // 其实，打印长度也可以是运行时再确定，就是要在后面加上美元符号告诉编译器你要设置长度
    println!("{number:🧧>width$}", number=1, width=5);

    // 在1.58+版本中，可以方便的直接引用变量进行打印
    let number: i32 = 1;
    let width: usize = 5;
    println!("{number:🌞>width$}");
    // 按理说，constant的作用域是全局，所以自然也可以直接引用
    println!("{FOO}");

    // FIXME
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // TODO
    #[allow(dead_code)]
    struct Structure(i32);
    impl std::fmt::Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // write! 这个宏返回类型为 std::fmt::Result
            write!(f, "{}", self.0)
        }
    }
    println!("This struct `{}` won't print...", Structure(3));

    // Decimal
    // 用 小数点 + 数字 n 来表示要打印小数点后 n 位
    let pi: f32 = 3.141592;
    println!("Pi is routhly {pi:.3}");
}