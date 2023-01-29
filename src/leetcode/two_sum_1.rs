use std::collections::HashMap;

pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32,usize> = HashMap::new();
    for (e, &i) in nums.iter().enumerate() {
        let difference = target-i;
        if let Some(&j) = map.get(&difference) {
            return vec![j as i32,e as i32];
        }
        else {
            map.insert(i,e);
        }
    }
    panic!("Solution not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_two_sum() {
        let vect: Vec<i32> = vec![2,3,5,1];
        const target: i32 = 4;
        
        let ideal_result: Vec<i32> = vec![1,3];
        let test_result: Vec<i32> = two_sum_1(vect,target);

        assert_eq!(test_result,ideal_result);

    }

    #[test]
    fn test_2_two_sum() {
        let vect: Vec<i32> = vec![2,1,8,3,4];
        const target: i32 = 6;
        
        let ideal_result: Vec<i32> = vec![0,4];
        let test_result: Vec<i32> = two_sum_1(vect,target);

        assert_eq!(test_result,ideal_result);

    }
}