///2025-11-25
struct Solution;
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut sum = 1;
        let mut i = 1;
        let mut rests = std::collections::HashSet::new();
        loop {
            let rest = sum % k;
            if rest == 0 {
                return i;
            }
            if rests.contains(&rest) {
                return -1;
            }
            rests.insert(rest);
            sum = sum * 10 + 1;
            sum %= k;
            i += 1;
        }
    }
}
fn main() {
    for k in 1..10001 {
        let result = Solution::smallest_repunit_div_by_k(k);
        println!("Result: {:?}", result);
    }
}
