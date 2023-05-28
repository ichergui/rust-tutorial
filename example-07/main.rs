mod quick;
mod selection;

fn main() {
    let mut array = vec![10, -2, 6, -4, 3, -1];
    println!("Input array: {:?}", array);
    quick::sort(&mut array);
    println!("Sorted array: {:?}", array);
    assert_eq!(array, [-4, -2, -1, 3, 6, 10]);

    array = vec![10, -2, 6, -4, 3, -1];
    println!("Input array: {:?}", array);
    selection::sort(&mut array);
    println!("Sorted array: {:?}", array);
    assert_eq!(array, [-4, -2, -1, 3, 6, 10]);
}