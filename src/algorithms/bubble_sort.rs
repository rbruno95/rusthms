// function to sort an array of integers using bubble sort
pub fn sort(arr: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut temp;
    while i < arr.len() {
        while j < arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}
