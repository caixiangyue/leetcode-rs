// Given an array of integers, return indices of the two numbers such that they add up to a specific target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// Example:

// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (k, v) in nums.iter().enumerate() {
            match map.get(&(target - v)) {
                None => { map.insert(v, k); }
                Some(sub) => return vec![*sub as i32, k as i32],
            }
        }
        vec![]
    }
}

#[cfg(test)]    
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(v, Solution::two_sum(vec![10], 5));
    }
}