
fn qsort(vec: &mut[i32]) {
    // do nothing.
}

#[cfg(test)]
mod tests{
    use super::qsort;

    #[test]
    pub fn test_qsort(){
        let mut vec = vec![1, 2, 3];

        qsort(&mut vec);
        assert_eq!(vec, [1, 2, 3]);
    }
}