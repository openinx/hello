use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug)]
pub struct SomeOne {
    name: String,
    value: u32,
}

impl Display for SomeOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.value)
    }
}

impl Ord for SomeOne {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.value, &self.name).cmp(&(other.value, &other.name))
    }
}

impl PartialOrd for SomeOne {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SomeOne {
    fn eq(&self, other: &Self) -> bool {
        (self.value, &self.name) == (other.value, &other.name)
    }
}

impl Eq for SomeOne {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ord() {
        let s1 = SomeOne {
            name: "Jack".to_owned(),
            value: 23,
        };
        let s2 = SomeOne {
            name: "Mike".to_owned(),
            value: 20,
        };

        assert_eq!("(Jack, 23)", format!("{}", s1));
        assert_eq!("(Mike, 20)", format!("{}", s2));

        assert_eq!(s1, s1);
        assert_eq!(s1 > s2, true);
        assert_eq!(s2 < s1, true);
        assert_eq!(s1 >= s2, true);
        assert_eq!(s2 <= s1, true);
        assert_eq!(s2 != s1, true);
        assert_eq!(s1 != s2, true);
    }
}
