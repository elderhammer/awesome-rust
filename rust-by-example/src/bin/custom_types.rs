#![allow(dead_code)]
fn main() {
    println!("{:=^56}", " main ");

    structures();

    enums();

    linked_list();

    fn structures() {
        println!("{:=^56}", " structures ");

        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8
        }

        // Unit
        struct Unit;

        // Tuple struct
        struct Pair(i32, f32);

        // Classic
        struct Point {
            x: f32,
            y: f32
        }

        struct Rectangle {
            top_left: Point,
            bottom_right: Point
        }

        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };
        println!("{:?}", peter);

        let point: Point = Point { x: 10.3, y: 0.4 };
        println!("x: {}, y: {}", point.x, point.y);

        // struct 语法糖：更新语法，或者叫点点语法
        let bottom_right = Point { x: 5.2, ..point };
        println!("x: {}, y: {}", bottom_right.x, bottom_right.y);

        // 解构
        let Point { x: left_edge, y: top_edge } = point;

        let rectangle = Rectangle {
            top_left: Point { x: left_edge, y: top_edge },
            bottom_right
        };

        let _unit = Unit;

        let pair = Pair(1, 0.1);
        println!("pair contains {:?} and {:?}", pair.0, pair.1);
        // 解构
        let Pair(integer, decimal) = pair;
        println!("pair contains {:?} and {:?}", integer, decimal);

        // 嵌套解构
        // 第一遍写错了，以为纯结构声明就行，结果要通过 field 名字匹配
        fn rect_area(rectangle: &Rectangle) -> f32 {
            let Rectangle {
                top_left: Point { x: x1, y: y2 },
                bottom_right: Point { x: x2, y: y1 }
            } = rectangle;

            (x2 - x1) * (y2 - y1)
        }

        println!("the area of rect: {}", rect_area(&rectangle));

        fn square(point: Point, edge: f32) -> Rectangle {
            Rectangle {
                bottom_right: Point { x: point.x + edge, y: point.y - edge },
                top_left: point,
            }
        }

        let point: Point = Point { x: 10.3, y: 0.4 };
        let edge = 10.3f32;
        let square = square(point, edge);
        println!("the area of rect: {}", rect_area(&rectangle));
    }

    fn enums() {
        println!("{:=^56}", " enums ");

        clike();

        fn clike() {
            println!("{:=^56}", " c like ");

            // discriminator ？枚举中，每个变体都对应一个整数值，默认是按序从0开始
            // 目的：在编译期，通过 discriminator 来区分变体，从而确定对应变体及其成员变量，从而确定内存空间？
            enum Numbe {
                One = 1,
                Two = 2,
            }

            let one = Numbe::One;
            println!("one is {}", one as i32);
        }
    }

    fn linked_list() {
        println!("{:=^56}", " linked list ");

        use List::*;

        enum List {
            Cons(u32, Box<List>),
            Nil
        }

        impl List {
            fn new() -> List {
                Nil
            }

            fn prepend(self, elem: u32) -> List {
                Cons(elem, Box::new(self))
            }

            fn len(&self) -> u32 {
                match self {
                    Nil => 0,
                    Cons(_, next) => 1 + next.len()
                }
            }

            fn stringify(&self) -> String {
                match self {
                    Nil => format!("Nil"),
                    Cons(elem, next) => format!("{}, {}", elem, next.stringify())
                }
            }
        }

        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}