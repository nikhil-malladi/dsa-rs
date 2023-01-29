pub fn is_palindrome_1(s: String) -> bool {
        let chars: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).collect();
        let lowercase_chars: Vec<char> = chars.into_iter().map(|c| c.to_lowercase().next().unwrap()).collect();
        lowercase_chars == lowercase_chars.iter().rev().cloned().collect::<Vec<char>>()
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_is_palindrome() {
        let st: String = String::from("A man, a plan, a canal: Panama");
        
        let ideal_result: bool = true;
        let test_result: bool = is_palindrome_1(st);

        assert_eq!(test_result,ideal_result);

    }

    #[test]
    fn test_2_is_palindrome() {
        let st: String = String::from("race a car");
        
        let ideal_result: bool = false;
        let test_result: bool = is_palindrome_1(st);

        assert_eq!(test_result,ideal_result);

    }
}