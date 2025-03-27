use std::{collections::HashMap};

#[warn(dead_code)]
pub struct Solution;

impl Solution {
    pub fn group_anagrams(list: Vec<String>) -> Vec<Vec<String>> {
        let mut memory = HashMap::new();
        for item in list {
            let mut count = Vec::new();
            for ch in item.chars()  {
                count.push(ch as u8 - 'a' as u8);
            }
            memory.entry(count).or_insert(Vec::new()).push(item);
        }
        memory.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn group_anagram() {
        
    }

}
