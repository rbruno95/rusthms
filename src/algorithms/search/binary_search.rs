pub fn binary_search_recusrive(arr: &Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
    if left > right {
        return -1;
    }
    let mid = (left + right) / 2;
    if arr[mid as usize] == target {
        return mid;
    }
    if arr[mid as usize] > target {
        return binary_search_recusrive(arr, target, left, mid - 1);
    }
    return binary_search_recusrive(arr, target, mid + 1, right);
}

pub fn binary_search_iterative(arr: &Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = (arr.len() - 1) as i32;
    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid as usize] == target {
            return mid as i32;
        }
        if arr[mid as usize] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return -1;
}
