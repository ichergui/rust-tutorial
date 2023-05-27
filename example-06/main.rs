fn main () {
    let _i:u64;
    let _j:u64;
    let mut arr = [10, -2, 6, -4, 3, -1];

    for _i in 0..arr.len() {
        for _j in _i..arr.len() {
            if arr[_j] < arr[_i] {
                arr.swap(_j, _i);
            }
        }
    }
    println!("{:?}", arr);
}
