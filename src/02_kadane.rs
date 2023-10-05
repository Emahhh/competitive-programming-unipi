// https://leetcode.com/problems/maximum-subarray/
struct Solution{

}

impl Solution {

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::max;

        let mut max_subarr_sum : i32;
        let mut last_max_sum : i32;

        last_max_sum = nums[0];
        max_subarr_sum = nums[0];

        for i in 1..nums.len() {
            last_max_sum = max(nums[i], last_max_sum + nums[i]);
            max_subarr_sum = max(max_subarr_sum, last_max_sum);
        }
        
        return max_subarr_sum;
    }
}

fn main() {
    println!("{}", Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
}