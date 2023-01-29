use std::collections::HashMap;

pub fn valid_parenthesis_1(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let hm: HashMap<char,char> = HashMap::from([(')','('),(']','['),('}','{')]);

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            _ => {
                if stack.iter().last() == hm.get(&c) {
                    stack.pop();
                }
                else {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_valid_parenthesis() {
        let s: String = String::from("({[]})");
        
        let ideal_result: bool = true;
        let test_result: bool = valid_parenthesis_1(s);

        assert_eq!(test_result,ideal_result);

    }

    #[test]
    fn test_2_valid_parenthesis() {
        let s: String = String::from("{[}]");
        
        let ideal_result: bool = false;
        let test_result: bool = valid_parenthesis_1(s);

        assert_eq!(test_result,ideal_result);

    }
}