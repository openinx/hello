
fn _sort_(v: &mut[i32], l: usize, r: usize) {
    let mut i = l;
    let x = v[r];
    for j in l..r {
        if v[j] <= x {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, r);

    if l < i {
        _sort_(v, l, i-1);
    }
    if i < r {
        _sort_(v, i+1, r);
    }
}

pub fn qsort(v: &mut[i32]) {
    if v.len() <= 1{
        return;
    }
    _sort_(v, 0, v.len() - 1);
}

#[cfg(test)]
mod tests{
    use super::qsort;

    #[test]
    pub fn test_qsort(){
        let mut vec = vec![1, 2, 3];
        qsort(&mut vec);
        assert_eq!(vec, [1, 2, 3]);

        vec = vec![3, 2, 1];
        qsort(&mut vec);
        assert_eq!(vec, [1, 2, 3]);

        vec = vec![1, 1, 1];
        qsort(&mut vec);
        assert_eq!(vec, [1, 1, 1]);

        vec = vec![2, 8, 4, 5, 7, 2, 5];
        qsort(&mut vec);
        assert_eq!(vec, [2, 2, 4, 5, 5, 7, 8]);

        vec = vec![];
        qsort(&mut vec);
        assert_eq!(vec, []);
    }
}