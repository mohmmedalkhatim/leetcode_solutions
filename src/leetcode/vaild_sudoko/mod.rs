#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    pub fn vaild_sudoko(board: Vec<Vec<&str>>) -> bool {
        let mut row: HashSet<&str> = HashSet::new();
        let mut col: HashSet<&str> = HashSet::new();
        let mut boxs: HashMap<(usize, usize), &str> = HashMap::new();
        for i in 0..9 {
            for j in 0..9 {
                let element = board[i][j];
                if board[i][j] == "." {
                    continue;
                }
                if row.contains(element)
                    || col.contains(element)
                    || (boxs.get(&(i / 3, j / 3)).unwrap() == &element)
                {
                    return false;
                }
                row.insert(element);
                col.insert(element);
                boxs.entry((i / 3, j / 3)).or_insert(element);
            }
        }
        return true;
    }

    #[test]
    fn valid_sudoko_test() {
        vaild_sudoko(vec![Vec::new()]);
    }
}
