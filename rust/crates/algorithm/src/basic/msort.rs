// Merge sort for an integer array.
fn _msort(v: &mut [i32], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let m = (l + r) >> 1;
    _msort(v, l, m);
    _msort(v, m + 1, r);

    // Temporary array to cache the whole merge sorted elements.
    let mut w = Vec::with_capacity(r + 1 - l);
    let mut i = l;
    let mut j = m + 1;
    while i <= m && j <= r {
        if v[i] <= v[j] {
            w.push(v[i]);
            i += 1;
        } else {
            w.push(v[j]);
            j += 1;
        }
    }

    while i <= m {
        w.push(v[i]);
        i += 1;
    }

    while j <= r {
        w.push(v[j]);
        j += 1;
    }

    // Copy the elements back to the vec
    for i in 0..(r + 1 - l) {
        v[l + i] = w[i];
    }
}

pub fn merge_sort(v: &mut [i32]) {
    if v.len() > 1 {
        _msort(v, 0, v.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_msort() {
        let mut vec = vec![];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![]);

        vec = vec![-1];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![-1]);

        vec = vec![3, 2, 1];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);

        vec = vec![8, 2, 4, 1, 3, 4];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 4, 8]);

        vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
