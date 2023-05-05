pub fn binary_search_1(arr: &[i32],target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        // println!("left: {left}, right: {right}");
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        }
        else if arr[mid] < target {
            left = mid + 1
        }
        else if arr[mid] > target {
            right = mid - 1
        }
    }
    None
}

