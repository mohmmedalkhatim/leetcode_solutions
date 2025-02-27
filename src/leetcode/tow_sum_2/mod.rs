use std::collections::HashMap;

fn tow_sum_2(arr: Vec<i32>, target: i32) -> Vec<usize> {
    let mut start = 0 as usize;
    let mut end = arr.len() - 1;
    while start < end {
        let sum = arr[start] + arr[end];
        if sum == target {
            return vec![start + 1, end + 1];
        }
        if sum > target {
            end -= 1;
            continue;
        }
        if sum < target {
            start += 1;
            continue;
        }
    }
    vec![1,3]
}
#[cfg(test)]
mod test {
    use super::tow_sum_2;
    #[test]
    fn tow_sum_2_test() {
        assert_eq!(tow_sum_2(vec![2,7,11,12],9),vec![1,2]);
    }
}