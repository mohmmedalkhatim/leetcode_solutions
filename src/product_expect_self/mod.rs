use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn product_expect_self(arr: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = arr.clone();
        let mut prefix = 1;
        for (i, v) in arr.iter().enumerate() {
            res[i] = prefix;
            prefix *= v;
        }
        let mut postfix = 1;
        for (i, v) in arr.iter().enumerate().rev() {
            res[i] *= postfix;
            postfix *= v;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn product_expect_self() {
        assert_eq!(
            Solution::product_expect_self(Vec::from(vec![2, 3, 4, 5])),
            vec![60, 40, 30, 24]
        )
    }
}
