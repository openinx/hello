
pub fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }
    
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    
    true
}

#[cfg(test)]
mod tests{
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_prime(){
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(53), true);
        assert_eq!(is_prime(2<<31 - 1), false);
    }
}