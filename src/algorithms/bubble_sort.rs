use std::mem;

// function to sort an array of integers using bubble sort
pub fn sort(arr: &mut [i32]) {
    let mut temp: i32;
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}
