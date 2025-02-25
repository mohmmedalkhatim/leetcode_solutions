use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn top_k(arr: Vec<i32>, k: usize) -> Vec<i32> {
        let mut memory = HashMap::new();
        let mut freq = Vec::new();
        for i in arr.clone() {
            freq.push(Vec::new());
        }
        for item in arr {
            *memory.entry(item).or_insert(0) += 1;
        }
        for (n, c) in memory {
            freq[c].push(n);
        }
        let mut res = Vec::new();
        for item in (0..freq.len() - 1).rev() {
            for i in freq[item].clone().into_iter() {
                res.push(i);
                if res.len() == k {
                    return res;
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
    fn top_k() {
        
    }
}
