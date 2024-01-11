fn main() {
    println!("{:=^56}", " main ");

    raii();

    ownership_and_moves();

    borrowing();
}

fn raii() {
    println!("{:=^56}", " raii ");

    // RAII：有一种读法叫资源获取即初始化，但是感觉反过来讲更好记：（变量）初始化即完成资源的获取。
    // 大意是，资源的有效期和与资源绑定的变量生命周期一致，即变量初始化的时候即完成了资源的获取，变量离开作用域调用析构时完成资源的释放
    // 好处是，只要正确的析构，就不会出现资源泄露。

}

fn ownership_and_moves() {
    println!("{:=^56}", " ownership and moves ");

    // 栈上分配资源的例子
    let x = 5i32;

    // i32 实现了 copy，所以没有发生 move
    let y = x;

    println!("{}, {}", x, y);

    // 堆上分配资源的例子
    let a = Box::new(1);

    // Box 没有实现 copy，所以发生了 move
    let b = a;

    // 发生了 move，原变量不能再访问
    //println!("moved {}", a);

    mutability();

    partical_moves();

    fn mutability() {
        println!("{:=^56}", " mutability ");

        // 😮数据，注意是数据的可变性可以通过 move 来改变
        let immutable_box = Box::new(0);

        let mut mutable_box = immutable_box;

        *mutable_box = 1;
        println!("box: {}", mutable_box);
    }

    fn partical_moves() {
        // 除了 tuple、array 之外，struct 竟然也可以部分引用
        // tuple、array 在部分引用之后，会导致整体失去 W、O 权限，关键是引用
        // struct 解构的时候也可以部分引用，同时应用所有权和借用规则，会导致整体失去 R、W、O 权限
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }

        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        let Person { name, ref age } = person;

        // 按照所有权规则，move 之后不能在访问
        //println!("{:?}", person);

        println!("{:?}", person.age);

        let mut foo = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        fn what(person: &Person) -> &Box<u8> {
            &person.age
        }

        //let Person { ref mut age, ..} = foo;
        //**age = 2;
        let a = what(&foo);

        foo.name.push_str("what");

        println!("{:?}", foo.name);
        //println!("{:?}", a);

        // 编译器不会根据声明来确定具体的元素，直接认为任意元素发生了借用
        fn get_first(name: &(Box<i32>, Box<i32>, String)) -> &Box<i32> {
            &name.0
        }

        let mut name = (
            Box::new(1),
            Box::new(2),
            String::from("bar")
        );

        //let bos = get_first(&name);
        let bos = &name.0;

        //let first = &name.0;

        //name.2.push_str("what");
        //*name.1 = 3;

        println!("{:?}", name);
        println!("{:?}", bos);
    }
}

fn borrowing() {
    println!("{:=^56}", " borrowing ");

    fn eat_box_i32(bos: Box<i32>) {
        println!("Eat box i32: {:?}", bos);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("Borrow i32: {:?}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5i32);
    let stacked_i32 = 6i32;

    // 借用不会发生 move
    // 没想到编译器可以通过类型推断来自动解引用？！
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        borrow_i32(_ref_to_i32);
    }

    // 传值发生 move
    eat_box_i32(boxed_i32);

    mutability();

    aliasing();

    fn mutability() {
        println!("{:=^56}", " mutability ");

        #[derive(Clone, Copy)]
        struct Book {
            author: &'static str,
            title: &'static str,
            year: u32,
        }

        fn borrow_book(book: &Book) {
            println!("I immutably borrowed {} - {} edition", book.title, book.year);
        }

        fn new_edition(book: &mut Book) {
            book.year = 2014;
            println!("I mutably borrowed {} - {} edition", book.title, book.year);
        }

        let immutabook = Book {
            author: "Douglas Hofstadter",
            title: "Gödel, Escher, Bach",
            year: 1979,
        };

        // 通过 move 来改变可变性
        let mut mutabook = immutabook;

        borrow_book(&immutabook);

        borrow_book(&mutabook);

        //new_edition(&mut immutabook);

        new_edition(&mut mutabook);
    }

    fn aliasing() {
        println!("{:=^56}", " aliasing ");

        struct Point { x: i32, y: i32, z: i32 }

        let mut point = Point { x: 0, y: 0, z: 0 };

        let borrowed_point = &point;
        let another_borrow = &point;

        println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

        // 怎么理解 "same time"：变量初始化那行 -> 最后一次调用那行，如果存在两个变量有交叉重叠的情况，就意味着 "same time"
        //let mutable_borrow = &mut point;
    
        println!("Point has coordinates: ({}, {}, {})",
                    borrowed_point.x, another_borrow.y, point.z);

        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;
        println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
    }
}