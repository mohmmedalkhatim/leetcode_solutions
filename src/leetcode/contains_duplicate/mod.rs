use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hashset = HashSet::new();
        for num in nums {
            match hashset.get(&num) {
                Some(_) => return true,
                None => {
                    hashset.insert(num.clone());
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_duplicate() {
        let resoult_1 = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        let resoult_2 = Solution::contains_duplicate(vec![1, 2, 3, 4]);
        assert!(resoult_1);
        assert_eq!(resoult_2,false);
      }
}
