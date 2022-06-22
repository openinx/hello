fn hash(s: &[u8]) -> i64 {
    let mut res = 0;
    for c in s {
        // f(n+1) = 33 * f(n) + c;
        res = _mod(_mod(res * 33) + (*c as i64));
    }
    return res;
}

fn _mod(x: i64) -> i64 {
    // mod is 2^31 -1 = 2147483647, which is a prime.
    ((x % 2147483647) + 2147483647) % 2147483647
}

fn str_cmp(a: &[u8], b: &[u8]) -> bool {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn substr(p: &[u8], s: &[u8]) -> i32 {
    if p.len() == 0 {
        return -1;
    }
    if s.len() == 0 {
        return 0;
    }
    if s.len() > p.len() {
        return -1;
    }

    let target = hash(s);
    let mut val = hash(&p[0..s.len()]);
    if val == target && str_cmp(&p[0..s.len()], s) {
        return 0;
    }

    // t = 33^(m-1), where m is the length of child string.
    let mut t = 1;
    for _ in 1..s.len() {
        t = _mod(t * 33);
    }

    for i in 1..(p.len() - s.len() + 1) {
        // Hash(n) = (Hash(n-1) - p[n-1] * mod^(m-1)) * mod + p[n+m-1];
        // where n is the length of parent string, and m is the length of child string.
        val =
            _mod(_mod(_mod(val - _mod((p[i - 1] as i64) * t)) * 33) + (p[i + s.len() - 1] as i64));

        if val == target && str_cmp(&p[i..i + s.len()], s) {
            return i as i32;
        }
    }

    return -1;
}

fn str_substr(p: &str, s: &str) -> i32 {
    return substr(p.as_bytes(), s.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basic() {
        assert_eq!(str_substr("abaab", "baa"), 1);
        assert_eq!(str_substr("acabaabaabcacaabc", "abaabc"), 5);
        assert_eq!(str_substr("a", "a"), 0);
        assert_eq!(str_substr("ba", "a"), 1);
        assert_eq!(str_substr("bab", "ab"), 1);
        assert_eq!(str_substr("bab", "abc"), -1);
        assert_eq!(str_substr("babaaaaaaabbaa", "abbaa"), 9);
        assert_eq!(str_substr("abbaa", "babaaaaaaabbaa"), -1);
        assert_eq!(str_substr("aaaabbbabbbaaacccabacaaaabbaaaab", "baaaa"), 26);
    }
}
