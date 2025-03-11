fn binary_search(arr: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start <= end {
        let mid = (end + start) / 2;

        if arr[mid] < target {
            start = mid + 1;
        } else if arr[mid] > target {
            end = mid - 1;
        } else if arr[mid] == target {
            return target;
        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::binary_search;
    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(vec![1, 2, 3, 4, 5], 4), 4);
        assert_ne!(binary_search(vec![1, 2, 3, 4, 5], 4), 5);
    }
}
