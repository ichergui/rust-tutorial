fn sum_arr(_arr: &[i64])-> i64 {
    let mut res = 0;
    for i in 0.._arr.len() {
        res += _arr[i];
    }
    res
}

fn main () {
    let arr = [10, -2, 6, -4];
    let first_elem = arr[0];
    println!("First element: {}", first_elem);

    for i in 0..arr.len() {
        println!("element[{}] = {}", i, arr[i]);
    }

    let res = sum_arr(&arr);
    println!("sum_arr = {}", res);
}
