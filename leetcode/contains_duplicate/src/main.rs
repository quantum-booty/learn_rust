use std::collections::HashSet;

fn main() {
    assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    assert!(Solution::contains_duplicate(vec![
        1, 1, 1, 3, 3, 4, 3, 2, 4, 2
    ]));
}

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::<i32>::new();
        for n in nums {
            if set.contains(&n) {
                return true;
            }
            set.insert(n);
        }
        false
    }
}
