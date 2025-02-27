use std::{collections::HashMap, string};

#[warn(dead_code)]
fn valid_anagram(s: String, mut t: String) -> bool {
    return true;
}
#[warn(dead_code)]
fn valid_anagram_sort(s: String, t: String) -> bool {
    let mut a: Vec<&str> = s.split("").collect();
    let mut b: Vec<&str> = t.split("").collect();
    a.sort();
    b.sort();
    if a == b {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {

}
