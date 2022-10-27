
struct Solution;

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![3, 3], 6));
}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let required_map: HashMap<i32, i32> = nums
            .iter()
            .enumerate()
            .map(|(idx, n)| (*n, idx as i32))
            .collect();
        for (idx1, n) in nums.into_iter().enumerate() {
            let idx1 = idx1 as i32;
            if let Some(idx2) = required_map.get(&(target - n)) {
                if idx1 == *idx2 {
                    continue;
                }
                return vec![idx1, *idx2];
            }
        }
        vec![]
    }
}
