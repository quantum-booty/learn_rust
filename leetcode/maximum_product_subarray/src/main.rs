fn main() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![0, 2]), 2);
    assert_eq!(Solution::max_product(vec![3, -1, 4]), 4);
    assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
}

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_prod = *nums.iter().max().unwrap();
        let mut min = 1;
        let mut max = 1;
        for n in nums {
            if n == 0 {
                min = 1;
                max = 1;
                continue;
            }

            let old_max = max;
            max = (n * max).max(n * min).max(n);
            min = (n * old_max).min(n * min).min(n);
            max_prod = max_prod.max(max);
        }
        max_prod
    }
}
