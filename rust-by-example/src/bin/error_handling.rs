fn main() {
    println!("{:=^56}", " error handling ");

    painc();

    abort_and_unwind();

    option_and_unwrap();

    result();

    multiple_error_types();

    iterating_over_results();
}

fn painc() {
    println!("{:=^56}", " panic ");

    drink("water");
    //drink("lemonade");

    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            panic!("AAAaaaaaa!!!!!");
        }

        println!("Some refreshing {} is all I need.", beverage);
    }
}

// abort 和 unwind 是 Rust 中处理 panic 的不同方式
// abort 在 panic 发生的时候，会立即终止程序，内存的释放、关闭文件等由 OS 负责
// unwind 除了会回滚调用栈（析构）还有回调机制，可以设计 catch 逻辑来执行一些清理逻辑
fn abort_and_unwind() {
    println!("{:=^56}", " abort and unwind ");
}

fn option_and_unwrap() {
    println!("{:=^56}", " option and unwrap ");

    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing: Option<&str> = None;

    drink(coffee);
    //drink(nothing);

    // 利用 match 显式处理 Option
    // 一般是给调用方兜底的机会
    fn give_adult(drink: Option<&str>) {
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner)   => println!("{}? How nice.", inner),
            None          => println!("No drink? Oh well."),
        }
    }

    // 利用 unwrap 或者 expect 隐式处理 Option
    // 一般在写 test、prototype 等情况可以隐式处理 Option
    fn drink(drink: Option<&str>) {
        let inside = drink.unwrap();
        if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }
    
        println!("I love {}s!!!!!", inside);
    }

    unpacking_options_with_q();
    
    // ? 看上去是隐式的，但是显式的（隐式，指没有提供完整的错误处理逻辑，例如 panic），因为在结果是 None 的时候，不是panic，而是 return None
    fn unpacking_options_with_q() {
        println!("{:=^56}", " unpacking options with q ");

        struct Person {
            job: Option<Job>,
        }
        
        #[derive(Clone, Copy)]
        struct Job {
            phone_number: Option<PhoneNumber>,
        }
        
        #[derive(Clone, Copy)]
        struct PhoneNumber {
            area_code: Option<u8>,
            number: u32,
        }

        impl Person {
            // Gets the area code of the phone number of the person's job, if it exists.
            fn work_phone_area_code(&self) -> Option<u8> {
                self.job?.phone_number?.area_code
            }
        }

        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    number: 439222222,
                }),
            }),
        };
    
        assert_eq!(p.work_phone_area_code(), Some(61));
    }

    combinators_map();

    // 怎么理解 combinator 呢？
    fn combinators_map() {
        println!("{:=^56}", " combinators: map ");

        //#![allow(dead_code)]

        #[derive(Debug)] enum Food { Apple, Carrot, Potato }
        
        #[derive(Debug)] struct Peeled(Food);
        #[derive(Debug)] struct Chopped(Food);
        #[derive(Debug)] struct Cooked(Food);
        
        // 类似这样的映射真的会有很多吗？似乎从来没写过
        fn peel(food: Option<Food>) -> Option<Peeled> {
            match food {
                Some(food) => Some(Peeled(food)),
                None       => None,
            }
        }

        fn chop(food: Option<Peeled>) -> Option<Chopped> {
            match food {
                Some(Peeled(food)) => Some(Chopped(food)),
                None       => None,
            }
        }

        fn cook(food: Option<Chopped>) -> Option<Cooked> {
            // map: Option<T> -> Option<U>
            // 函数形式：|T| U
            food.map(|Chopped(food)| Cooked(food))
        }

        fn process(food: Option<Food>) -> Option<Cooked> {
            food.map( |food| Peeled(food) )
                .map( |Peeled(food)| Chopped(food) )
                .map( |Chopped(food)| Cooked(food) )
        }

        fn eat(food: Option<Cooked>) {
            match food {
                Some(food) => println!("Mmm. I love {:?}", food),
                None       => println!("Oh no! It wasn't edible."),
            }
        }

        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));
        // Let's try the simpler looking `process()` now.
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }

    combinators_and_then();

    // and_then 方法的入参是前序 Option 的 T，返回值是一个 Option<U>
    // 相比之下，map 会在 f 的返回值上加一层 Option，比如 f 返回 Option<T> 的时候，map 会返回 Option<Option<T>>
    fn combinators_and_then() {
        println!("{:=^56}", " combinators: and then ");

        #[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
        #[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

        fn have_ingredients(food: Food) -> Option<Food> {
            match food {
                Food::Sushi => None,
                _           => Some(food),
            }
        }

        fn have_recipe(food: Food) -> Option<Food> {
            match food {
                Food::CordonBleu => None,
                _                => Some(food),
            }
        }

        fn cookable_v1(food: Food) -> Option<Food> {
            match have_recipe(food) {
                None       => None,
                Some(food) => match have_ingredients(food) {
                    None       => None,
                    Some(food) => Some(food),
                },
            }
        }

        fn cookable_v2(food: Food) -> Option<Food> {
            have_recipe(food).and_then(have_ingredients)
        }

        fn eat(food: Food, day: Day) {
            match cookable_v2(food) {
                Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
                None       => println!("Oh no. We don't get to eat on {:?}?", day),
            }
        }

        let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

        eat(cordon_bleu, Day::Monday);
        eat(steak, Day::Tuesday);
        eat(sushi, Day::Wednesday);
    }

    or();

    fn or() {
        println!("{:=^56}", " or ");

        #[derive(Debug)] 
        enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

        let apple = Some(Fruit::Apple);
        let orange = Some(Fruit::Orange);
        let no_fruit: Option<Fruit> = None;

        // 这种算是默认值吗？
        let first_available_fruit = no_fruit.or(orange).or(apple); // 按照所有权的设计，运行时才能确定的逻辑一律认为 move
        println!("first_available_fruit: {:?}", first_available_fruit);
    }

    or_else();

    fn or_else() {
        println!("{:=^56}", " or else ");

        #[derive(Debug)] 
        enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

        let apple = Some(Fruit::Apple);
        let no_fruit: Option<Fruit> = None;
        let get_kiwi_as_fallback = || {
            println!("Providing kiwi as fallback");
            Some(Fruit::Kiwi)
        };
        let get_lemon_as_fallback = || {
            println!("Providing lemon as fallback");
            Some(Fruit::Lemon)
        };

        let first_available_fruit = no_fruit
            .or_else(get_kiwi_as_fallback)
            .or_else(get_lemon_as_fallback);
        println!("first_available_fruit: {:?}", first_available_fruit);
        println!("lemon: {:?}", first_available_fruit);
    }

    // 从源码看
    // 即时执行的特点是入参是 T，在链式调用的过程中，不管条件是否满足，T 都 move 了
    // 延迟执行的特点是入参是 Fn，在链式调用的过程中，只有条件满足，才会调用 Fn，否则不会
    // 那举一反三，判断条件是：是否根据条件来执行 Fn。
    get_or_insert();

    fn get_or_insert() {
        println!("{:=^56}", " get or insert ");

        #[derive(Debug)]
        enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

        let mut my_fruit: Option<Fruit> = None;
        let apple = Fruit::Apple;
        // 这个函数也是即时执行
        let first_available_fruit = my_fruit.get_or_insert(apple);
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit);
    }

    get_or_insert_with();

    fn get_or_insert_with() {
        println!("{:=^56}", " get or insert with ");

        #[derive(Debug)]
        enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

        let mut my_fruit: Option<Fruit> = None;
        // 惰性执行，或者叫延迟执行
        let first_available_fruit = my_fruit.get_or_insert_with(|| {
            Fruit::Lemon
        });
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit);
    }
}

fn result() {
    println!("{:=^56}", " result ");

    map_for_result();

    fn map_for_result() {
        println!("{:=^56}", " map for result ");

        use std::num::ParseIntError;

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
            })
        }

        let twenty = multiply("10", "2");
        print(twenty);

        let tt = multiply("t", "2");
        print(tt);
    }

    introducing_q();

    fn introducing_q() {
        println!("{:=^56}", " introducing ? ");
        use std::num::ParseIntError;

        type AliasedResult<T> = Result<T, ParseIntError>;

        fn print(result: AliasedResult<i32>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
            let first_number = first_number_str.parse::<i32>()?;

            let second_number = second_number_str.parse::<i32>()?;

            Ok(first_number * second_number)
        }

        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }

}

fn multiple_error_types() {
    println!("{:=^56}", " multiple error types ");

    defining_an_error_type();

    // 基本的思路是定一个结构体来表示错误，尽量不要携带提示用的字符串信息，可以通过实现 Display 来传达字符串
    fn defining_an_error_type() {
        println!("{:=^56}", " defining an error type ");

        use std::fmt;

        type Result<T> = std::result::Result<T, DoubleError>;

        #[derive(Debug, Clone)]
        struct DoubleError;

        impl fmt::Display for DoubleError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            vec.first()
                .ok_or(DoubleError)
                .and_then(|s|{
                    s.parse::<i32>()
                        .map_err(|_| DoubleError)
                        .map(|i| 2 * i)
                })
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }

    boxing_errors();

    fn boxing_errors() {
        println!("{:=^56}", " boxing errors ");

        use std::fmt;
        use std::error;

        type E = Box<dyn error::Error>;
        type Result<T> = std::result::Result<T, E>;

        #[derive(Debug, Clone)]
        struct EmptyVec;

        impl fmt::Display for EmptyVec {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "empty vec")
            }
        }

        // 利用 Box 的关键点，就是实现相应的 trait
        impl error::Error for EmptyVec {}

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            vec.first()
                .ok_or_else(|| EmptyVec.into())
                .and_then(|s|{
                    s.parse::<i32>()
                        .map_err(|e| e.into())
                        .map(|i| i * 2)
                })
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }

    other_uses_of_q();

    fn other_uses_of_q() {
        println!("{:=^56}", " other uses of ? ");

        use std::error;
        use std::fmt;

        type E = Box<dyn error::Error>;
        type Result<T> = std::result::Result<T, E>;

        #[derive(Debug, Clone)]
        struct EmptyVec;

        impl fmt::Display for EmptyVec {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "empty vec")
            }
        }

        impl error::Error for EmptyVec {}

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            // ok_or 的入参表示 E 的类型，？表示如果 E 和 函数的返回类型可以转换，将自动转换
            let first = vec.first().ok_or(EmptyVec)?;
            let parsed = first.parse::<i32>()?;

            Ok(parsed * 2)
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n)  => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

fn iterating_over_results() {
    println!("{:=^56}", " iterating over results ");

    // 结果类型是 Result
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);

    // 结果类型是 &'static str
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    let strings = vec!["tofu", "93", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s|s.parse::<u8>())
        // 利用 closure 来写过滤逻辑
        .filter_map(
            // 1.对的留下，错的不要 -> .ok()
            // 2.错的写到 errors -> closure -> map_err：对的原样返回，错的类型转换
            |r| r.map_err(|e|errors.push(e)).ok()
        )
        .collect();
    println!("Results: {:?}", numbers);

    // 另一种类型转换
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        // 这一步只是声明了一个 Map 类型
        .map(|s|s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}