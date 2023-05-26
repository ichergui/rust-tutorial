fn factorial (number: u64) -> u64 {
    let mut fact = 1;
    if number == 0 {
        fact = 1;
    }
    else {
        for i in 1..=number {
            fact *= i;
        }
    }
    return fact;
} 

fn factorial_0 (number: u64) -> u64 {
    let mut fact = 1;
    if number == 0 {
        fact = 1;
    }
    else {
        fact *= number*factorial_0(number-1); 
    }
    return fact;
}

fn fibonacci (number: u64) -> u64 {
    let fibo;
    if number == 0 {
        fibo = 0;
    }
    else {
        if number == 1 {
            fibo = 1;
        }
        else {
            fibo = fibonacci(number-1) + fibonacci(number-2);
        }
    }
    return fibo;
}

fn sqr(x: f64) -> f64 {
    return x * x;
}

fn main () {
    println!("sqr(2.0) = {}", sqr(2.0));

    println!("factorial(0) = {}", factorial(0));
    println!("factorial(5) = {}", factorial(5));

    println!("factorial_0(0) = {}", factorial_0(0));
    println!("factorial_0(5) = {}", factorial_0(5));

    println!("fibonacci(0) = {}", fibonacci(0));
    println!("fibonacci(9) = {}", fibonacci(9));
    assert_eq!(fibonacci(9), 34);
}
