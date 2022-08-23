extern "C" {
    fn rand() -> i32;
}

pub fn gen_i32() -> i32 {
    unsafe { rand() }
}

pub fn gen_bool() -> bool {
    unsafe { rand() & 1 != 0 }
}

pub fn gen_u32() -> u32 {
    unsafe { (rand() & 0x7FFFFFF) as u32 }
}

pub fn shuffle<T>(v: &mut Vec<T>) {
    let m = v.len();
    for i in 0..m {
        v.swap(i, gen_u32() as usize % m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_i32() {
        (0..1000_00).for_each(|_| {
            let val = gen_i32();
            assert!(val <= i32::MAX); // 2^31 - 1 ;
            assert!(val >= i32::MIN); // - 2^31   ;
        });
    }

    #[test]
    pub fn test_u32() {
        (0..1000_00).for_each(|_| {
            let val = gen_u32();
            assert!(val <= u32::MAX); // 2^32 - 1;
            assert!(val >= u32::MIN); // 0
        })
    }

    #[test]
    pub fn test_shuffle() {
        let mut v: Vec<usize> = (0..1000_0).collect();
        for _ in 0..10 {
            shuffle(&mut v);
        }

        let mut hit = vec![false; 1000_0];
        for i in 0..v.len() {
            let x = v[i];
            assert_eq!(hit[x], false);
            hit[x] = true;
        }

        assert_eq!(hit, vec![true; 1000_0]);
    }
}
