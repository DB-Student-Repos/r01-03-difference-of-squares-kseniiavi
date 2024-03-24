pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        sum += i
    }
    return sum * sum
    // unimplemented!("square of sum of 1...{n}")
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sums = 0;
    for i in 1..n {
        sums += i * i
    }
    return sums
    //unimplemented!("sum of squares of 1...{n}")
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
    
    //unimplemented!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
}
