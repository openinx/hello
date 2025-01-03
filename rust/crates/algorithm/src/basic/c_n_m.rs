fn print_vec(v: &Vec<bool>) {
    for i in 0..v.len() {
        if v[i] == false {
            print!("0");
        } else {
            print!("1");
        }
    }
    println!()
}

fn generate(v: &mut Vec<bool>, start_index: usize, remaining: usize) -> u32 {
    if remaining == 0 {
        print_vec(v);
        return 1;
    }

    let mut res = 0;
    for i in start_index..v.len() {
        v[i] = true;
        res += generate(v, i + 1, remaining - 1);
        v[i] = false;
    }
    res
}

fn c(n: usize, m: usize) -> u32 {
    let mut v = vec![false; n];
    generate(&mut v, 0, m)
}

fn calc(n: usize, m: usize) -> u32 {
    let mut res = 1u32;
    for i in 1..=m {
        res *= (n + 1 - i) as u32;
        res /= i as u32;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basic() {
        assert_eq!(3, c(3, 2));
        assert_eq!(4, c(4, 3));
        assert_eq!(10, c(5, 2));
        assert_eq!(252, c(10, 5));
        assert_eq!(252, calc(10, 5));
        assert_eq!(calc(10, 6), c(10, 6));
    }
}
