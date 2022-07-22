// function to sort an array of integers using bubble sort

#[allow(dead_code)]
pub fn sort(arr: &mut [i32]) {
    let n = arr.len();
    for _ in 0..n {
        for j in 0..n - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
