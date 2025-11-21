///2025-11-21
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if map.contains_key(num) {
                return vec![map[num] as i32, i as i32];
            }
            map.insert(target - num, i);
        }
        vec![]
    }
}
fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    println!("Result: {:?}", result);
}
