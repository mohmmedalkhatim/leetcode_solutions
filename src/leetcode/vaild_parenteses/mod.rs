use std::collections::HashMap;

fn vaild_parenteses(v: Vec<char>) -> bool {
    let mut map = HashMap::new();
    map.entry(')').or_insert('(');
    map.entry(']').or_insert('[');
    map.entry('}').or_insert('{');
    let mut stack = Vec::new();
    for item in v {
        match map.get(&item) {
            Some(s) => {
                if stack.is_empty() {
                    return false;
                }
            }
            None => {}
        }
        stack.push(item);
    }
    todo!()
}
