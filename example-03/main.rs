fn main () {
    let mut sum = 0;
    let mut sub = 30;

    for i in 0..10 {
        sum += i;
    }
    println!("sum = {}", sum);

    for j in 0..5 {
        sub -= j;
    }
    println!("sub = {}", sub);
}
