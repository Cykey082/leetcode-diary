///2025-12-01
struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}
fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = Solution::min_operations(nums, 3);
    println!("Result: {:?}", result);
}
