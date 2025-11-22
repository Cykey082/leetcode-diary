///2025-11-22
struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut str: Vec<u8> = Vec::new();
        let strs: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        for i in 0..strs.iter().map(|s| s.len()).min().unwrap_or(0) {
            let sample = strs[0][i];
            if strs.iter().skip(1).any(|s| s[i] != sample) {
                break;
            }
            str.push(sample);
        }
        String::from_utf8(str).unwrap()
    }
}
fn main() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let result = Solution::longest_common_prefix(strs);
    println!("Result: {:?}", result);
}
