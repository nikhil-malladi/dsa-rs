use std::collections::HashSet;

pub fn contains_duplicate_1(nums: Vec<i32>) -> bool {
    let mut h1 = HashSet::new();
    for num in nums.iter(){
        if h1.contains(&num) {
            return true;
        }
        else {
        h1.insert(num);
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contains_duplicate_1() {
        let vec: Vec<i32> = vec![1,2,3];
        let test_result: bool = contains_duplicate_1(vec);
        const ideal_result: bool = false;
        assert_eq!(test_result,ideal_result);
    }

    #[test]
    fn test_contains_duplicate_2() {
        let vec: Vec<i32> = vec![1,2,3,1];
        let test_result: bool = contains_duplicate_1(vec);
        const ideal_result: bool = true;
        assert_eq!(test_result,ideal_result);
    }
}
