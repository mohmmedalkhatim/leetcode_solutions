use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn tow_sum(nums: Vec<usize>, target: usize) -> Vec<usize> {
        let mut memory: HashMap<usize, usize> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match memory.get(&(target - v)) {
                Some(state) => {
                    return vec![*state, i];
                }
                None => {
                    memory.insert(*v, i);
                }
            }
        }
        Vec::new()
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn tow_sum(){
        assert_eq!(Solution::tow_sum(vec![1,2,3,4,5,6,7,8,9], 12),vec![4,6]);
        assert_ne!(Solution::tow_sum(vec![1,2,3,4,5,6,7,], 5),vec![1,8]);
    }
}
