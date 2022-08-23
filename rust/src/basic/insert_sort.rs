pub fn insert_sort<T: Ord>(v: &mut Vec<T>) {
    for i in 1..v.len() {
        for k in (0..i).rev() {
            if v[k] <= v[k + 1] {
                break;
            } else {
                v.swap(k, k + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut v: Vec<i32> = vec![];
        insert_sort(&mut v);
        assert_eq!(v, vec![]);

        v = vec![10];
        insert_sort(&mut v);
        assert_eq!(v, vec![10]);

        v = vec![1; 100];
        insert_sort(&mut v);
        assert_eq!(v, vec![1; 100]);

        v = vec![1, 2, 3];
        insert_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);

        v = vec![3, 2, 1];
        insert_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);

        v = vec![1, 10, 10, 10, 2, 3, 3, 2, 2, 2];
        insert_sort(&mut v);
        assert_eq!(v, vec![1, 2, 2, 2, 2, 3, 3, 10, 10, 10]);
    }
}
