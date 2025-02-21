struct Solution;

impl Solution {
    pub fn encode(arr: Vec<String>) -> String {
        let mut res = String::new();
        let mut count = 0;
        for item in arr.clone() {
            for i in item.chars() {
                res.push(i);
            }
            count += 1;
            if count != arr.len(){
                res.push('#');
            }

        }
        return res;
    }
    pub fn decode(str: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut word = String::new();
        let mut count = 0;
        for ch in str.chars() {
            count += 1;
            if ch !='#' {
                word.push(ch); 
            }
            if ch == '#' || count ==str.len() {
                res.push(word);
                word = String::new();
                continue;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn encode() {

        assert_eq!(
            "name#user#email".to_string(),
            Solution::encode(vec![
                "name".to_string(),
                "user".to_string(),
                "email".to_string()
            ])
        );
    }
    #[test]
    fn decode(){
        assert_eq!(
            Solution::decode("name#user#passoword".to_string()),
            vec![
                "name".to_string(),
                "user".to_string(),
                "passoword".to_string()
            ]
        );
    }
}
