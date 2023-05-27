
fn selection(array: &mut[i32]) {
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

fn main () {
    let mut arr = vec![10, -2, 6, -4, 3, -1];
    println!("Input array: {:?}", arr);
    selection(&mut arr);
    println!("Sorted array: {:?}", arr);
    assert_eq!(arr, [-4, -2, -1, 3, 6, 10])
}
