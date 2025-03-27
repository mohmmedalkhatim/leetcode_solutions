use std::collections::HashMap;

fn container_with_most_water(list: Vec<i32>)->i32 {
    let mut left = 0;
    let mut right = list.len() - 1;
    let mut index = list.len();
    let mut res :i32=0;
    while left < right {
        index -= 1;
        if left > right {
            left += 1;
        }
        if right > left {
            right -= 1;
        }
        res = res.max((left*right*index) as i32)
    }
    res
}
