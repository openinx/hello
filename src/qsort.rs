

fn _sort_(v: &mut[i32], l: i32, r: i32) {
    if l >= r {
        return;
    }

    let mid = (l + r) >> 1;
}

fn qsort(v: &mut[i32]) {
    // TODO: implement this method.
    println!("{:?}", v);
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