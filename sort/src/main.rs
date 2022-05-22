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

    /**
     * 测试快排
     */
    let mut nums5 = [74,4,1846,474346,11,728,44];
    let len = nums5.len();
    quick_sort(&mut nums5, 0, (len - 1) as usize);
    println!("quick sort: {:?}", nums5);

    /**
     * 测试插入排序
     */
    let mut nums6 = [74,4,1846,474346,11,728,44];
    insert_sort(&mut nums6);
    println!("insert sort: {:?}", nums6);
    let mut nums7 = [74,4,1846,474346,11,728,44];
    binsert_sort(&mut nums7);
    println!("binsert sort: {:?}", nums7);
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

/**
 * 快速排序：有自信的排序算法
 * 
 * 关键步骤：分而治之
 * 1.分区
 * 有左护法、右护法、中介，左右互相靠近并通过中介进行比较、交互指；
 * 2.递归小分区
 */

fn quick_sort(nums: &mut [i32], left: usize, right: usize) { // 刚开始只有一个大分区，要切成两半
    if left < right { // 说明数组长度大于1
        // 一分为二，同时split上的元素已就位
        let split = partition(nums, left, right);
        // 处理左分区
        if split > 1 {
            quick_sort(nums, left, split - 1);
        }
        // 处理右分区
        quick_sort(nums, split + 1, right);
    }
}

fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    // 用分区的第一个元素作为中介
    let split = left;

    // 左右标记
    let mut left_mark = left;
    let mut right_mark = right;

    /**
     * 循环主要是两种情况：
     * 1.左右标记互相靠近的过程中，发现了逆序的元素；
     * 2.left_mark > right_mark，即左标记越过了右标记，
     *  说明当前两个子区是“整体”有序的，左子区的元素都要小于等于中介，右子区的元素都要大于等于中介
     *  此时，将r返回作为新的中介
     */
    loop {
        // 左标记->
        while left_mark <= right_mark && nums[left_mark] <= nums[split] {
            left_mark += 1;
        }

        // <-右标记
        while left_mark <= right_mark && nums[split] <= nums[right_mark] {
            right_mark -= 1;
        }

        if left_mark <= right_mark {
            nums.swap(left_mark, right_mark)
        } else {
            break;
        }
    }

    nums.swap(split, right_mark);

    right_mark
}

/**
 * 插入排序
 */
fn insert_sort(nums: &mut [i32]) {
    for i in 1..nums.len() { // 从第二个元素开始
        // i表示待就位的元素，它的位置可能会被取代，所以用临时变量记下来
        let curr = nums[i];
        let mut pos = i; // pos用来找合适位置的

        /**
         * 向左移动的条件：
         * 1.直到碰到一个比curr小的元素；
         * 2.pos不能越界，所以pos要大于0；
         */
        while pos > 0 && curr < nums[pos - 1] {
            // 元素后移，准备给curr腾出位置
            nums[pos] = nums[pos - 1];

            pos -= 1; // 继续向左移动
        }
        nums[pos] = curr;
    }
}

/**
 * 插入排序（二分查找）
 */
fn binsert_sort(nums: &mut [i32]) {
    // 用二分查找来确定位置
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() { // 从第二个元素开始
        // i表示待就位的元素，它的位置可能会被取代，所以用临时变量记下来
        let curr = nums[i];

        left = 0;
        right = i - 1;
        while left <= right {
            mid = left + ((right - left) >> 1);
            if curr < nums[mid] {
                if 0 == mid {break;}
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        // 挪位置
        for j in (left..=i-1).rev() {
            nums.swap(j, j+1);
        }

        if left != i {
            nums[left] = curr;
        }
    }
}

