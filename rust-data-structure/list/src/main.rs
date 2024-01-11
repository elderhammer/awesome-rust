fn main() {
    println!("Hello, world!");

    /**
     * 测试链表
    fn basics() {
        let mut list = list::foo::List::new();
        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));
        list.peek_mut().map(|val| {
            *val = 4;
        });

        assert_eq!(list.peek(), Some(&4));
        println!("basics test Ok!");
    }

    fn into_iter() {
        let mut list = list::foo::List::new();
        list.push(1); list.push(2); list.push(3); let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        println!("into_iter test Ok!");
    }

    fn iter() {
        let mut list = list::foo::List::new();
        list.push(1); list.push(2); list.push(3); let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        println!("iter test Ok!");
    }

    fn iter_mut() {
        let mut list = list::foo::List::new();
        list.push(1); list.push(2); list.push(3); let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        println!("iter_mut test Ok!");
    }

    basics();
    into_iter();
    iter();
    iter_mut();
     */

    /**
     * 测试链表栈
     */
    let mut s = list::foo::Stack::new();
    s.push(1); s.push(2); s.push(4);

    println!("top {:?}, size {}",s.peek().unwrap(), s.size());
    println!("pop {:?}, size {}",s.pop().unwrap(), s.size());
    println!("is_empty:{}, stack:{:?}", s.is_empty(), s);
}