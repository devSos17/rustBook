pub fn fib(n: usize) -> (u128, usize) {
    match n {
        0 => (0, 1),
        1 => (1, 1),
        _ => {
            let (n1, cuenta1) = fib(n - 2);
            let (n2, cuenta2) = fib(n - 1);
            (n1 + n2, cuenta1 + cuenta2)
        }
    }
    // println!("Fib({n}) => {result}");
}

pub fn fib_ciclo(n: usize, cuenta: &mut usize) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: u128 = 0;
            let mut b: u128 = 1;
            let mut c: u128 = 0;
            for _i in 2..=n {
                *cuenta += 1;
                c = a + b;
                a = b;
                b = c;
            }
            c
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(fib(0).0, 0);
        assert_eq!(fib(1).0, 1);
        // assert_eq!(fib(-1).0, 0);
    }
    #[test]
    fn sequence() {
        assert_eq!(fib(5).0, 5);
        assert_eq!(fib(8).0, 21);
        assert_eq!(fib(9).0, 34);
        assert_eq!(fib(15).0, 610);
        assert_eq!(fib(19).0, 4181);
    }
    #[test]
    fn base_case_cic() {
        let mut cuenta = 0;
        assert_eq!(fib_ciclo(0, &mut cuenta), 0);
        assert_eq!(fib_ciclo(1, &mut cuenta), 1);
    }
    #[test]
    fn sequence_cic() {
        let mut cuenta = 0;
        assert_eq!(fib_ciclo(5, &mut cuenta), 5);
        assert_eq!(fib_ciclo(8, &mut cuenta), 21);
        assert_eq!(fib_ciclo(9, &mut cuenta), 34);
        assert_eq!(fib_ciclo(15, &mut cuenta), 610);
        assert_eq!(fib_ciclo(19, &mut cuenta), 4181);
    }
}
