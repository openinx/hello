fn _fixup(h: &mut [i32], len: usize) {
    let mut c = len; // child index.
    while c > 0 {
        // Parent index.
        let p = (c - 1) >> 1;
        if h[c] < h[p] {
            h.swap(c, p);
            c = p;
        } else {
            break;
        }
    }
}

fn _fixdown(h: &mut [i32], len: usize) {
    if len > 1 {
        h.swap(0, len - 1);

        let mut p = 0 as usize;
        let mut c = (p << 1) + 1;
        while c < len - 1 {
            if c + 1 < len - 1 && h[c + 1] < h[c] {
                c += 1;
            }

            if h[c] < h[p] {
                h.swap(c, p);
                p = c;
                c = (p << 1) + 1;
            } else {
                break;
            }
        }
    }
}

pub fn sort(v: &mut [i32]) {
    let mut h = Vec::with_capacity(v.len());

    for i in 0..v.len() {
        h.push(v[i]);
        _fixup(&mut h, i);
    }

    for i in 0..v.len() {
        v[i] = h[0];
        _fixdown(&mut h, v.len() - i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sort() {
        let mut v = vec![];
        sort(&mut v);
        assert_eq!(v, vec![]);

        v = vec![0];
        sort(&mut v);
        assert_eq!(v, vec![0]);

        v = vec![3, 2, 1];
        sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);

        v = vec![1, 2, 3];
        sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);

        v = vec![8, 2, 2, 4, 3, 1, 3];
        sort(&mut v);
        assert_eq!(v, vec![1, 2, 2, 3, 3, 4, 8]);
    }
}
