// https://leetcode.com/problems/maximum-subarray/

impl Solution {

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::max;

        let mut maxSubarrSum : i32;
        let mut lastMaxSum : i32;

        lastMaxSum = nums[0];
        maxSubarrSum = nums[0];

        for i in 1..nums.len() {
            lastMaxSum = max(nums[i], lastMaxSum + nums[i]);
            maxSubarrSum = max(maxSubarrSum, lastMaxSum);
        }
        
        return maxSubarrSum;
    }
}