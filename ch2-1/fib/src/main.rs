fn main() {
    println!("{}", fib(10));

    let mut memo = vec![0; 100];
    println!("{}", fib2(10, &mut memo));
}


fn fib(n: u32) -> u32 {
    if n <= 1 { return n }
    fib(n - 1) + fib(n - 2)
}


fn fib2(n: usize, memo: &mut Vec<u32>) -> u32 {
    if  n <= 1 { return n as u32 }
    if memo[n] != 0 { return memo[n] }
    memo[n] = fib2(n - 1, memo) + fib2(n - 2, memo);
    memo[n]
}
