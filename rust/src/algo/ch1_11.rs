#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::util;

    #[test]
    pub fn test() {
        let x = util::testdata_dir();
        assert_eq!(true, x.unwrap().exists());
    }
}
