use std::ops::Deref;

struct DerefExample<T> {
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test(){
        let x = DerefExample{value: 'a'};
        assert_eq!('a', *x);

        let x = DerefExample{value: 123};
        assert_eq!(123, *x);

        let x = DerefExample{value: 123.3};
        assert_eq!(123.3, *x);
    }
}