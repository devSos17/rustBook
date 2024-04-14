pub fn converter() {
    todo!();
}

fn c_to_f(c: f64) -> f64 {
    let res: f64 = (c * (9_f64 / 5_f64)) + 32_f64;
    round(res, 10_f64)
}

fn f_to_c(f: f64) -> f64 {
    let res: f64 = (f - 32_f64) * (5_f64 / 9_f64);
    round(res, 10_f64)
}

fn round(num: f64, precision: f64) -> f64 {
    (num * precision).round() / precision
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn convert_c() {
        // 21 C == 69.8 F
        assert_eq!(69.8, c_to_f(21_f64));
        // 25 C == 77 F
        assert_eq!(77_f64, c_to_f(25_f64));
    }
    #[test]
    fn convert_f() {
        // 21 C == 69.8 F
        assert_eq!(21_f64, f_to_c(69.8));
        // 25 C == 77 F
        assert_eq!(25_f64, f_to_c(77_f64));
    }
}
