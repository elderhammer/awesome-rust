fn main() {
    println!("Hello, world!");

    /**
     * 测试冒泡
     */
    let mut nums1 = [74,4,1846,474346,11,728,44];
    bubble_sort1(&mut nums1);
    println!("bubble sort1: {:?}", nums1);
    let mut nums2 = [74,4,1846,474346,11,728,44];
    bubble_sort2(&mut nums2);
    println!("bubble sort2: {:?}", nums2);
    let mut nums3 = [74,4,1846,474346,11,728,44];
    cocktail_sort(&mut nums3);
    println!("cocktail sort: {:?}", nums3);
    let mut nums4 = [74,4,1846,474346,11,728,44];
    cantbelieveitcansort(&mut nums4);
    println!("cbic sort: {:?}", nums4);
}

/**
 * 冒泡
 */
fn bubble_sort1(nums: &mut [i32]) { // 头一次碰到用了引用，所以不用返回
    // 一个元素的时候，不用排序
    if nums.len() < 2 {
        return;
    }

    // 2个元素的时候只需比较1次即可，所以n个元素需要比较n-1次
    for _i in 1..nums.len() { // _i代表冒泡次数
        for j in 0..nums.len()-1 { // 从第一个开始，检查到最后一个
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1)
            }
        }
    }
}

// 冒泡有个特点，每次冒泡都是将最大的元素（未就绪的）放到准确的位置
fn bubble_sort2(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    // 只要发现交换就会继续
    let mut sorted = false;
    for i in 1..nums.len() { // [1, n)，n个元素的数组，只需冒泡n-1次即可，最小的不用冒，会自动就位
        if sorted == true {
            // 上一次未发生交换，就是有序不用冒泡了
            return;
        }
        sorted = true;
        for j in 0..nums.len()-i { // 每次都能确定一个
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
                sorted = false
            }
        }
    }
}

/**
 * 鸡尾酒（双向冒泡）在处理一些特殊例子，效果显著。例如，在2n个元素的集合中，除第n+1和第n个元素外均有序。
 * 记住以下几点：
 * 1.i从0开始，只需执行n/2次，即i∈[0, n/2)
 * 2.左到右时，j属于[i, n-i-1)
 * 3.右到左时，j属于(i+1, n-i-1]，但是反向执行
 */
fn cocktail_sort(nums: &mut [i32]) {
    // 元素只有一个的时候无需处理
    if nums.len() < 2 {
        return;
    }

    let mut sorted = false;
    for i in 0..nums.len() >> 1 { // 要考虑3个元素的情况
        if sorted == true {
            return;
        }
        sorted = true;
        // 从左到右
        for j in i..nums.len()-i-1 { // j和下标有关
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
                sorted = false;
            }
        }
        // 右到左
        for j in (i+1..=nums.len()-i-1).rev() {
            if nums[j] < nums[j-1] {
                nums.swap(j, j-1);
                sorted = false;
            }
        }
    }
}

/**
 * 难以置信算法：不用管烦人的下标
 */
fn cantbelieveitcansort(nums: &mut [i32]) {
    /**
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j)
            }
        }
    }
    */
    for i in 1..nums.len() {
        // 这里的优化思路跟前面的一样，每次都可以将一个数放到确定的位置
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j)
            }
        }
    }
}
