fn partition(array: &mut [i64], l:isize, r:isize) -> isize {
    let pivot = array[r as usize];
    let mut i = l - 1;  
    for j in l..r {
        if array[j as usize] <= pivot {
            i = i + 1;
            array.swap(i as usize, j as usize);
        }
    }
    array.swap((i+1) as usize, r as usize);

    i + 1
}

fn quick_sort_partition(array: &mut [i64], l:isize, r:isize) {
    if l < r && r - l >= 1 {
        let pivot = partition(array, l, r);
        quick_sort_partition(array, l, pivot - 1);
        quick_sort_partition(array, pivot + 1, r);
    }
}

pub fn sort(array: &mut [i64]) {
    let l = 0;
    let r = array.len() - 1;
    quick_sort_partition(array, l, r as isize);
}
