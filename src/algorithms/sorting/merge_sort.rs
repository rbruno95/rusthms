#[allow(dead_code)]
pub fn merge(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32> {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut sorted = vec![];
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            sorted.push(left[left_index]);
            left_index += 1;
            continue;
        }
        sorted.push(right[right_index]);
        right_index += 1;
    }
    sorted = [&sorted[..], &left[left_index..], &right[right_index..]].concat();
    sorted
}
pub fn merge_sort(arr: &Vec<u32>) -> Vec<u32> {
    if arr.len() == 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    left = merge_sort(&mut left);
    right = merge_sort(&mut right);

    merge(&left, &right)
}
