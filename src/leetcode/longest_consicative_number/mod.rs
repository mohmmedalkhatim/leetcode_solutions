use core::num;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let memory:HashSet<i32> = nums.clone().into_iter().collect();
        let mut longest = 0;
        for n in nums  {
            if !memory.contains(&(n-1)) {
                let count = (n..).take_while(|x|memory.contains(x)).count();
                longest = count.max(longest)
            };
        };
        return longest as i32;
    }
}
