pub fn fib(n: isize) -> usize {
    if n < 1 {
        return 0;
    } else if n == 1 {
        return 1;
    };
    let result = fib(n - 1) + fib(n - 2);
    println!("Fib({n}) => {result}");
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(-1), 0);
    }
    #[test]
    fn sequence() {
        assert_eq!(fib(5), 5);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(15), 610);
        assert_eq!(fib(19), 4181);
    }
}
