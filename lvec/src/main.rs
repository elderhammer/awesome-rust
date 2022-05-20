fn main() {
    println!("Hello, world!");

    let mut lvec: lvec::foo::LVec<i32> = lvec::foo::LVec::new();
    lvec.push(10); lvec.push(11); lvec.push(12); lvec.push(13); lvec.insert(0,9);
    let mut lvec2: lvec::foo::LVec<i32> = lvec::foo::LVec::new();
    lvec2.insert(0, 8); lvec2.append(&mut lvec);
    println!("lvec2 len: {}", lvec2.len());
    lvec2.print_lvec();
    let res1 = lvec2.pop(); let res2 = lvec2.remove(0);
    println!("pop {:#?}", res1.unwrap());
    println!("remove {:#?}", res2.unwrap());
    lvec2.print_lvec();
}
