use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn vaild_palindroma(s: String) -> bool {
        let mut iter:Vec<char> = s.chars().filter(|c|c.is_alphanumeric()).map(|c|c.to_ascii_lowercase()).collect();
        iter.iter().eq(iter.iter().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn product_expect_self() {
        assert!(Solution::vaild_palindroma("ara".to_string()));
        assert!(!Solution::vaild_palindroma("arae".to_string()));
    }
}
