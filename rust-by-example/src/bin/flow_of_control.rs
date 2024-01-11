fn main() {
    println!("{:=^56}", " main ");

    if_else();

    looop();

    for_and_iterators();

    matches();
}

fn if_else() {
    println!("{:=^56}", " if else ");

    let n = 5;

    // 和其它语言相比，差异就是 Rust 中 if 条件不需要括号
    // 还有，因为 if 是一个 expression，所以要保证每一个分支的返回值类型统一
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2
        };

    println!("{} -> {}", n, big_n);
}

fn looop() {
    println!("{:=^56}", " looop ");

    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue; // 转向
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break; // 转向
        }
    }

    nesting_and_labels();

    returning_from_loops();

    fn nesting_and_labels() {
        println!("{:=^56}", " nesting and labels ");

        'outer: loop {
            println!("Entered the outer loop");
    
            'inner: loop {
                println!("Entered the inner loop");
    
                // 跳出当前循环
                //break;
    
                // 跳出外层循环
                break 'outer;
            }    
            //println!("This point will never be reached");
        }
        println!("Exited the outer loop");
    }

    fn returning_from_loops() {
        println!("{:=^56}", " returning from loops ");

        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                // 利用 break 返回数据
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }
}

fn for_and_iterators() {
    println!("{:=^56}", " for and iterators ");

    let names = vec!["Bob", "Frank", "Ferris"];

    // 默认情况下，for loop 会调用目标集合的 into_iter 方法，从而消费了目标集合
    // 当然，可以通过显式调用来调整，例如 iter、iter_mut
    for name in names.iter() {
        match name {
            // 这个模式匹配6
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}

fn matches() {
    println!("{:=^56}", " matches ");

    let number = 13;

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special")
    }

    destructuring_pointers_ref();

    binding();

    fn destructuring_pointers_ref() {
        println!("{:=^56}", " destructuring pointers ref ");

        let reference = &4;

        // 先匹配，再解构
        match reference {
            &val => println!("Got a value via destructuring: {:?}", val)
        }

        // 先解引用，再匹配
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val)
        }

        // 利用 ref 获取引用
        let ref value = 5;
        match value {
            &num => println!("{:?}", num)
        }

        // 匹配的时候利用 ref 创建一个引用
        match value {
            ref num => println!("{:?}", *num)
        }

        // 解构：利用对应的结构（形式）去匹配需要的元素
    }

    fn binding() {
        println!("{:=^56}", " binding ");

        fn age() -> u32 {
            15
        }

        match age() {
            0             => println!("I haven't celebrated my first birthday yet"),
            n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
            n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
            n => println!("I'm an old person of age {:?}", n),
        }
    }
}

