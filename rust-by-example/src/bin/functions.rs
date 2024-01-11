fn main() {
    println!("{:=^56}", " main ");

    fizzbuzz_to(100);

    methods();

    closures();

    higher_order_functions();

    diverging_functions();
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn methods() {
    println!("{:=^56}", " methods ");

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0)
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    struct Point {
        x: f64,
        y: f64
    }
    impl Point {
        fn origin() -> Point {
            Point {
                x: 0.0,
                y: 0.0
            }
        }

        fn new(x: f64, y: f64) -> Point {
            Point {
                x,
                y
            }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point
    }
    impl Rectangle {
        fn area(&self) -> f64 {
            // 解构还能这样写。。。
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
    
            self.p1.y += y;
            self.p2.y += y;
        }
    }

    struct Pair(Box<i32>, Box<i32>);
    impl Pair {
        fn destroy(self) {
            let Pair(first, second) = self;
            println!("Destroying Pair({}, {})", first, second);
        }
    }
}

fn closures() {
    // 原来，居中是向上的箭头
    println!("{:=^56}", " closures ");

    let outer_var = 42;

    // 完整的定义
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // 但其实可以借助编译器来自动推断类型
    let closure_inferred = |i| { i + outer_var };

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // 要注意的是，一旦通过类型推断，closure的参数类型就确定了，不能再利用编译器进行二次推断
    // 换句话说，在一个作用域内，closure的类型只能确定一次
    //println!("closure_inferred: {}", closure_inferred(1f64));

    let one = || 1;
    println!("closure returning one: {}", one());

    capturing();

    as_input_parameters();

    type_anonymity();

    input_functions();

    as_output_parameters();

    examples_in_std();

    fn capturing() {
        // 原来，居中是向上的箭头
        println!("{:=^56}", " capturing ");

        // 用 closure 的大部分时间都是直接 move 变量，没怎么关注过 borrow
        use std::mem;

        let color = String::from("green");
        // closure 捕获了 color 的不可变引用
        let print = || println!("`color`: {}", color);
        // 直到 closure 离开作用域，否则将一直持有 color 的不可变引用
        print();

        let _reborrow = &color;
        print();

        // 怎么理解离开作用域呢？
        // 直觉上认为是离开了所属的代码块{}
        // 不过，官方的解释是“直到最后一次调用”。
        let _color_move_ = color;

        let mut count = 0;
        // 没想到捕获了可变引用还要自身也是可变，因为通过可变引用修改了变量意味着 closure 内部发生了变化。
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        inc();

        // 由于下面的逻辑还有调用 inc ，所以无法获得 count 的引用
        // let _reborrow = &count;

        inc();

        let mut dec = || {
            count -= 1;
            println!("`count`: {}", count);
        };

        // 所以，要识别此类问题，就看多次引用的作用域是否交叉了
        //dec();
        //inc();

        // 不过在最后一次调用后，可以正常操作，再次获取 count 的可变引用
        let _count_reborrowed = &mut count;

        // Box 没有实现 Copy
        let movable = Box::new(3);

        // 由于是调用 drop ，又因为没有实现 Copy，所以会捕获所有权，也就是发生了 move
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };

        consume();
        // 通过所有权的规则，禁止了非法的内存访问逻辑
        //consume();
    }

    fn as_input_parameters() {
        println!("{:=^56}", " as_input_parameters ");

        // 类比上面 capturing 中对 closure 的类型进行声明 mut
        // 对入参是 closure 类型来说，Fn、FnMut、FnOnce 表达了可能对其内部变量的操作分别是只读、修改、获取。

        fn apply<F>(f: F) where
            F: FnOnce() {
            f()
        }

        fn apply_to_3<F>(f: F) -> i32
            where F: Fn(i32) -> i32 {
            f(3)
        }

        use std::mem;

        // &'static str
        let greeting = "hello";

        // String
        let mut farewell = "bye".to_owned();

        let daily = || {
            // 不可变引用就够了
            println!("I said {}.", greeting);

            // 需要可变引用
            farewell.push_str("!!!");
            println!("The I screamed {}.", farewell);

            // 需要获取所有权
            mem::drop(farewell);

            // 综上，这个 closure 需要声明为 FnOnce
        };

        apply(daily);

        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));

        // 注意，如果捕获的过程中，实际发生了 Copy 而不是 Move，那么 closure 实际类型为 Fn 而不是 FnOnce
        let mut count = 1;
        //let mut count = Box::new(1);
        let foo = |x| {
            mem::drop(count);
            mem::drop(x);
            1
        };
        apply_to_3(foo);
    }

    fn type_anonymity() {
        println!("{:=^56}", " type anonymity ");

        fn apply<F>(f: F) 
            where F: Fn() {
            f();
        }

        let x = 7;
        // 创建一个匿名类型，然后传入 x，并实现 Fn，最后赋值给 print
        let print = || {
            println!("{x}");
        };

        apply(print);
    }

    fn input_functions() {
        println!("{:=^56}", " input functions ");

        fn apply<F>(f: F) where
            F: Fn() {
            f();
        }

        fn function() {
            println!("I'm a function!");
        }

        let closure = || { println!("I'm a closure!"); };

        apply(function);
        apply(closure);
    }

    fn as_output_parameters() {
        println!("{:=^56}", " as output parameters ");

        fn create_fn() -> impl Fn() {
            let x = "foo".to_owned();

            move || println!("{}", x)
        }

        fn create_fn_mut() -> impl FnMut() {
            let x = "foo".to_owned();

            move || println!("{}", x)
        }

        fn create_fn_once() -> impl FnOnce() {
            let x = "foo".to_owned();

            move || println!("{}", x)
        }

        let c_fn = create_fn();
        // 记住，当声明为 FnMut 说明执行会改变 closure 内部状态，所以必须以 mut 调用，就好比变量一样
        let mut c_fn_mut = create_fn_mut();
        let c_fn_once = create_fn_once();

        c_fn();
        c_fn_mut();
        c_fn_once();
    }

    fn examples_in_std() {
        println!("{:=^56}", " examples in std ");

        iterator_any();

        searching_through_iterators();

        use std::mem;
        fn iterator_any() {
            println!("{:=^56}", " iterator any ");

            let vec1 = vec![1, 2, 3];
            let vec2 = vec![4, 5, 6];
            // 不太能理解这块？
            println!("{}", vec1.iter().any(|&x| { x == 2 } ));
            println!("{}", vec2.into_iter().any(|x| { x == 2 } ));

            let arr1 = [1, 2, 3];
            let arr2 = [4, 5, 6];

            // Fn、FnMut、FnOnce 声明是针对 closure 捕获的外部变量
            // Fn、FnMut、FnOnce 的入参不想干，举个例子，Fn 的入参可以是传值的形式，会发生 Move
            let foo = Box::new(1);
            let far = Box::new(2);
            let print = |foo| {
                mem::drop(foo);
                println!("{:?}", far);
            };

            print(foo);

            let mut count = "foo".to_owned();

            let print = || {
                println!("{}", count)
            };

            let mut bar = || {
                count.push_str("!!!");
            };

            // &T -> T
            // &T -> &mut T
            // &T -> &mut T -> &T ❌
            // &mut T -> &T
            // &mut T -> &T -> &mut T ❌
            // &mut T -> T
            //print();
            //let mut foo = &count;
            //bar()
        }

        fn searching_through_iterators() {
            println!("{:=^56}", " searching through iterators ");

            let vec1 = vec![1, 2, 3];
            let vec2 = vec![4, 5, 6];

            // 通过模式匹配来解构
            // 直觉上，会利用 * 来解引用
            // 实际上，也可以用模式匹配来解
            println!("Find 2 in vec1: {:?}", vec1.iter().find(|&x| *x == 2));
            println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));

            let arr1 = [1, 2, 3];
            let arr2 = [4, 5, 6];

            println!("Find 2 in arr1: {:?}", arr1.iter().find(|&&x| x == 2));
            println!("Find 2 in arr2: {:?}", arr2.into_iter().find(|&x| x == 2));
        }
    }
}

fn higher_order_functions() {
    println!("{:=^56}", "higher order functions");

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 函数式风味
    // 这种链式风格要怎么实现呢？
    // 源码看起来，好比状态转换，输入是谓词条件，输出是满足谓词的元素组成的新集合，以此类推
    let sum: u32 = (0..)
        // 遍历操作
        .map(|n| n * n)
        // 停止条件
        .take_while(|&n_squared| n_squared < upper )
        // 过滤
        .filter(|&n_squared| is_odd(n_squared) )
        .sum();
    println!("functional style: {}", sum);

    // 常规写法
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
}

fn diverging_functions() {
    println!("{:=^56}", "diverging functions");

    sum_odd_numbers(1000);

    // 与其叫“发散”函数，还不如叫“方向”函数，或者更直接点，叫“转向”函数
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => break,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
