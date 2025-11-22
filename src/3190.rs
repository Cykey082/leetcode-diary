///2025-11-22
struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter().map(|&x| (x % 3 != 0) as i32).sum()
    }
}
fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = Solution::minimum_operations(nums);
    println!("Result: {:?}", result);
}
