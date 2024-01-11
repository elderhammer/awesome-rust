fn main() {
    tuples();

    arrays_and_slices();
}

fn arrays_and_slices() {
    use std::mem;

    // 从 slice 的结构来看，一个指针，一个长度，说明只能由连续内存的对象创建而来
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    // 和 tuple 不同，array 是通过下标访问的
    println!("first element of array: {}", xs[0]);
    println!("second element of array: {}", xs[1]);

    // 到底占用了多少字节呢？
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 按照前面的分析呢，array 是一个连续内存的对象，所以，只要加上引用符号&，就可以创建一个 slice
    analyze_slice(&xs);

    // 也可以根据 array 的部分来创建 slice
    // 区间映射支持闭区间
    analyze_slice(&ys[1..4]);

    // 真没想到 array 还有 get 方法，返回类型是 Option
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far", i)
        }
    }

    fn analyze_slice(slice: &[i32]) {
        println!("first element of slice: {}", slice[0]);
        println!("length of slice: {}", slice.len());
    }
}

fn tuples() {
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (int_param, bool_param) = pair;
    
        (bool_param, int_param)
    }
    
    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }
    
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    
    impl std::fmt::Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
        }
    }
    
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // 打印前两个value
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 复合tuple，即value也是tuple。注意，tuple没有自动实现fmt::display
    let tuple_of_tuples = ((1u8, 2u16), (3u32, 4u64));
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 标准库中已实现的display最多支持12个元素，多了就要自己实现了
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);

    // 除了unit，任何tuple都包含至少一个逗号
    println!("one element tuple: {:?}", (5,));
    println!("just an integer: {:?}", (5));

    // 模式匹配：解构
    let tuple = (1, 2, 3, 4);
    let (a, b, c, d) = tuple;
    println!("{a}, {b}, {c}, {d}");

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("transpose:\n{}", transpose(matrix));
}