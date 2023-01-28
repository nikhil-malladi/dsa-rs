use std::collections::HashMap;

pub fn is_anagram_1(s: String, t: String) -> bool {

    if s.len()!=t.len() {
        return false;
    }
    let mut sm = HashMap::new();
    let mut tm = HashMap::new();

    for c in s.chars() {
        *sm.entry(c).or_insert(0) +=1;
    }

    for c in t.chars() {
        *tm.entry(c).or_insert(0) +=1;
    }
    sm == tm
}

pub fn is_anagram_2(s: String, t: String) -> bool {
    let mut s_sorted: Vec<char> = s.chars().collect();
    let mut t_sorted: Vec<char> = t.chars().collect();

    s_sorted.sort();
    t_sorted.sort();

    s_sorted == t_sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_1() {
        let st1: String = String::from("anagram");
        let st2: String = String::from("naaramg");

        const ideal_result: bool = true;
        let test_result_1: bool = is_anagram_1(st1.clone(),st2.clone());
        let test_result_2: bool = is_anagram_2(st1,st2);

        assert_eq!(test_result_1,ideal_result);
        assert_eq!(test_result_2,ideal_result);
    }

    #[test]
    fn test_is_anagram_2() {
        let st1: String = String::from("abcd");
        let st2: String = String::from("abab");

        const ideal_result: bool = false;
        let test_result_1: bool = is_anagram_1(st1.clone(),st2.clone());
        let test_result_2: bool = is_anagram_2(st1,st2);

        assert_eq!(test_result_1,ideal_result);
        assert_eq!(test_result_2,ideal_result);
    }
}