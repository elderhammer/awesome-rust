fn main() {
    println!("Hello, world!");

    /**
     * 测试数字求和
     */
    let nums = vec![1,2,3,4,5];
    let sum = nums_sum(&nums);
    println!("数组求和结果：{}", sum);

    /**
     * 测试尾递归
     */
    let nums = vec![1,2,3,4,5];
    println!("尾递归数组求和结果：{}", nums_sum_tail(0, &nums));

    /**
     * 测试进制转换
     */
    println!("进制转换结果：{}", base_convert(100, 16));

    /**
     * 测试汉诺塔
     */
    move2tower(3, "left", "mid", "right");

    /**
     * 测试动态规划
     */
    let amount = 6;
    let mut changed_caches: [u32; 10] = [0; 10];
    let change_times = dp_rec_mc(&[1, 5, 10], amount, &mut changed_caches);
    println!("{}找零结果：{}", amount, change_times);
}

/**
 * 递归求解数字数组的和
 */
fn nums_sum(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum(&nums[1..])
    }
}

/**
 * 尾递归求解数组和：每次都计算前两个值的结果，然后进入下一个计算
 */
fn nums_sum_tail(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        nums_sum_tail(sum + nums[0], &nums[1..])
    }
}

const BASETER: [&str; 16] = ["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];
/**
 * 递归进制转换
 */
fn base_convert(num: i32, base: i32) -> String {
    if 0 == num {
        "".to_string()
    } else {
        base_convert(num / base, base) + BASETER[(num % base) as usize]
    }
}

/**
 * 递归解汉若塔
 * 关键是，用具体例子，观察过程找到过程的重复规律，而不要盯着两个、三个、四个、N个塔的总过程
 * 技巧：画图，记住以下3步，设n>=3
 * 1.将n-1个盘从left移到mid；
 * 2.将第n个盘从left移到right；
 * 3.由第一步可知，可将n-1个盘子从mid移到right；
 * 4.结束；
 * 特殊情况有两种，n=1、2的时候，兼容处理即可
 */
fn move2tower(height: i32, left_p: &str, mid_p: &str, right_p: &str) { // q.要怎么抽象塔和柱子呢？a.通过画图来理解
    if height >= 1 {
        // 第一步
        move2tower(height - 1, left_p, right_p, mid_p);
        // 第二步
        println!("move no.{} plate from {} to {}", height, left_p, right_p);
        // 第三步
        move2tower(height - 1, mid_p, left_p, right_p);
    }
}

/**
 * 动态规划：找零
 * 特点：
 * 1.从小问题到大问题；
 * 2.大问题复用小问题的结果，避免重复计算？
 * 3.小问题可以得到最优解？
 * 第2点能不能做到决定了能不能用动态规划来解决问题！？
 */
fn dp_rec_mc(caches: &[u32], amount: u32, changed_caches: &mut [u32]) -> u32 {
    for denm in 1..=amount { // 注意迭代顺序，以便大问题复用小问题的解
        // 如果用全用一块来找，就可以拿到最“多”钱
        let mut min_change = amount;
        for c in caches.iter()
                       .filter(|&c| *c <= denm) // 只能用面额小于等于的钱来找
                       .collect::<Vec<&u32>>() { // q.不熟悉vec的iter、filter、collect方法
            // 如果以一张c来找，那么接下去查剩余的钱要找多少次就可以了
            let remain = denm - c;

            // 所以，找零总次数等于用一张c加上剩余数额的找零次数
            let change_num = 1 + changed_caches[remain as usize];

            // 注意，能用于找零的钱可能有多张，要逐个比较找到最少的次数
            if change_num < min_change {
                min_change = change_num
            }
        }

        // 记录当前数额的最少找零次数，可被更大数额（大问题）的找零问题复用
        changed_caches[denm as usize] = min_change;
    }

    changed_caches[amount as usize]
}