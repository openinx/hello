struct MapReduce<T> {
    input: Vec<T>,
    map: Option<Box<dyn Map>>,
    reduce: Option<Box<dyn Reduce>>,
}

impl<T> MapReduce<T> {
    fn new(input: Vec<T>) -> Self {
        MapReduce {
            input,
            map: None,
            reduce: None,
        }
    }

    fn map<I, O, F>(&mut self, map: &dyn Map) {
        // FIXME: Still don't get the correct approach to assign the
        // `map` to `self.map`  :-(
        todo!()
    }

    fn reduce<I, O, F>(&mut self, reduce: &dyn Reduce) {
        // FIXME: Still don't get the correct approach to assign the
        // `reduce` to `self.reduce`  :-()
        todo!()
    }
}

trait Map {
    fn map<I, O>(x: I) -> O
    where
        Self: Sized;
}

trait Reduce {
    fn reduce<I, O>(a: I, b: I) -> O
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn word_count() {
        let mut words = vec![
            "hello", "world", "hello", "thanks", "world", "thanks", "world",
        ];

        let mut mr = MapReduce::new(words);
    }
}
