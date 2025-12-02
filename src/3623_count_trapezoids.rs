///2025-12-02
// 3623. 统计梯形的数目 I
// 1.使用Hashmap统计所有y坐标出现的次数，如果只有一个y坐标则返回0
// 2.对于每个y坐标，任取两点的组合的可能数，若少于两个可能数则返回0
// 3.对于所有可能数，乘以自己以外的总可能数，求和。
struct Solution;
impl Solution {
    pub fn cn2(n: i32) -> i32 {
        let n: i64 = n as i64;
        let m = 2_000_000_014;
        (n * (n - 1) / 2) as i32 % m
    }
    pub fn mul(a: i32, b: i32) -> i32 {
        let m = 2_000_000_014;
        ((a as i64) * (b as i64) % (m as i64)) as i32
    }
    pub fn add(a: i32, b: i32) -> i32 {
        let m = 2_000_000_014;
        (((a as i64) + (b as i64)) % (m as i64)) as i32
    }
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut y_count = std::collections::HashMap::new();
        for point in &points {
            *y_count.entry(point[1]).or_insert(0) += 1;
        }
        if y_count.len() <= 1 {
            return 0;
        }
        let mut res = 1;
        let mut y_cnt = 0;
        let total_possible_num: i32 = y_count.values().map(|&cnt| Self::cn2(cnt)).sum();
        for &cnt in y_count.values() {
            let possible_num = Self::cn2(cnt);
            if possible_num >= 1 {
                let other_possible_num = total_possible_num - possible_num;
                res = Self::add(res, Self::mul(possible_num, other_possible_num));
            }
            y_cnt += 1;
        }
        if y_cnt < 2 {
            return 0;
        }
        res / 2
    }
}
fn main() {
    let points = vec![
        vec![-73, -72],
        vec![-1, -56],
        vec![-92, 30],
        vec![-57, -89],
        vec![-19, -89],
        vec![-35, 30],
        vec![64, -72],
    ];
    let result = Solution::count_trapezoids(points);
    println!("Result: {:?}", result);
}
