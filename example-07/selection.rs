pub fn sort(array: &mut [i64]) {
    let _i:u64;
    let _j:u64;
    for _i in 0..array.len() {
        for _j in _i..array.len() {
            if array[_j] < array[_i] {
                array.swap(_j, _i);
            }
        }
    }
}