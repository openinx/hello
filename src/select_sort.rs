pub fn select_sort<T>(v: &mut [T])
where
    T: Ord,
{
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            if v[j] < v[i] {
                v.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::time::Instant;

    #[test]
    pub fn test_i32_sort() {
        let mut v = vec![-1, -2, -3];
        select_sort(&mut v);
        assert_eq!(v, [-3, -2, -1]);

        v = vec![5, 1, 2, 3, 4];
        select_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);

        v = vec![];
        select_sort(&mut v);
        assert_eq!(v, []);
    }

    #[test]
    pub fn test_str_sort() {
        let mut v = vec!["B", "A", "C"];
        select_sort(&mut v);
        assert_eq!(v, ["A", "B", "C"]);

        v = vec!["E", "FFF", "KK", "EE", "EA"];
        select_sort(&mut v);
        assert_eq!(v, ["E", "EA", "EE", "FFF", "KK"]);

        v = vec!["", "A", "B"];
        select_sort(&mut v);
        assert_eq!(v, ["", "A", "B"]);
    }

    #[test]
    pub fn test_rng_float_sort() {
        let now = Instant::now();

        let mut rng = thread_rng();

        let mut v = [0 as i32; 1000];
        for i in 0..v.len() {
            v[i] = rng.gen();
        }

        select_sort(&mut v);

        for i in 1..v.len() {
            assert_eq!(v[i] >= v[i - 1], true);
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}
