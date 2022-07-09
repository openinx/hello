macro_rules! return_if_error {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let ret = $fn($($arg,)*);
        if ret != 0 {
            panic!("Return code is: {}", ret);
        } else {
            return ret;
        }
    }};
}

macro_rules! any_zero {
    () => {false};
    ($($x:expr),+ $(,)?) => {
        [$($x),*].iter().any(|i| *i == 0)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    #[should_panic(expected = "Return code is: 1")]
    pub fn test_panic() {
        let foo = || 1;
        let bar = || return_if_error!(foo());
        bar();
    }

    #[test]
    pub fn test_ok() {
        let foo = || 0;
        let bar = || return_if_error!(foo());
        assert_eq!(0, bar());
    }

    #[test]
    pub fn test_any_zero() {
        assert_eq!(false, any_zero!());
        assert_eq!(false, any_zero!(1, 2, 3));
        assert_eq!(true, any_zero!(1, 3, 4, 0));
        assert_eq!(false, any_zero!(-1, -1, -1));
    }
}
