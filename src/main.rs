fn main() {
    leetcode01()
}

fn leetcode01() {
    let nums: Vec<i32> = [3, 2, 4].to_vec();
    let target = 6;
    let a = leetcode::leetcode01::two_sum(nums, target);
    println!("{:?}", a);
}
