fn main() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    assert_eq!(Solution::max_sub_array(vec![-3, -2, 0, -1]), 0);
    assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
}

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut cur_sum = 0;
        for n in nums {
            if cur_sum < 0 {
                cur_sum = 0;
            }
            cur_sum += n;
            max_sum = max_sum.max(cur_sum);
        }
        max_sum
    }
}
