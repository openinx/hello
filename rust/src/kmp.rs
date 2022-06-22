/**
 * The Knuth–Morris–Pratt algorithm.
 * Please see: https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm
 */

fn gen_next(s: &[u8]) -> Vec<i32> {
    if s.len() == 0 {
        return Vec::new();
    }
    if s.len() == 1 {
        return vec![-1];
    }

    let mut next: Vec<i32> = (0..s.len()).map(|_| -1).collect();
    next[1] = 0;
    let mut i = 1;
    let mut j = 0;
    while i + 1 < s.len() {
        if s[i] == s[j] {
            i += 1;
            j += 1;
            next[i] = j as i32;
        } else if next[j] < 0 {
            i += 1;
            j = 0;
            next[i] = 0;
        } else {
            j = next[j] as usize;
        }
    }

    return next;
}

fn substr(p: &[u8], s: &[u8]) -> i32 {
    if p.len() == 0 {
        return -1;
    }
    if s.len() == 0 {
        return 0;
    }

    let next = gen_next(s);
    let mut i = 0;
    let mut j = 0;
    while i < p.len() && j < s.len() {
        if p[i] == s[j] {
            i += 1;
            j += 1;
        } else if next[j] < 0 {
            i += 1;
            j = 0;
        } else {
            j = next[j] as usize;
        }
    }

    if j == s.len() {
        return (i - j) as i32;
    } else {
        return -1;
    }
}

fn str_substr(p: &str, s: &str) -> i32 {
    return substr(&p.as_bytes(), &s.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_gen_next() {
        {
            let s = "aaaab";
            assert_eq!(gen_next(&s.as_bytes()), vec![-1, 0, 1, 2, 3]);
        }
        {
            let s = "abaabcac";
            assert_eq!(gen_next(&s.as_bytes()), vec![-1, 0, 0, 1, 1, 2, 0, 1]);
        }
        {
            let s = "abaabababbaaaabaab";
            assert_eq!(
                gen_next(&s.as_bytes()),
                vec![-1, 0, 0, 1, 1, 2, 3, 2, 3, 2, 0, 1, 1, 1, 1, 2, 3, 4]
            );
        }
    }

    #[test]
    pub fn test_substr() {
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
