use std::pin::Pin;

struct MapReduce<T> {
    input: Vec<T>,
    map: Option<Pin<Box<dyn Map>>>,
    reduce: Option<Pin<Box<dyn Reduce>>>,
}

impl<T> MapReduce<T> {
    fn new(input: Vec<T>) -> Self {
        MapReduce {
            input,
            map: None,
            reduce: None,
        }
    }

    fn map<F>(&mut self, map: F)
    where
        F: Map + 'static,
    {
        // FIXME: Still don't get the correct approach to assign the
        // `map` to `self.map`  :-(
        self.map = Some(Box::pin(map));
    }

    fn reduce<F>(&mut self, reduce: F)
    where
        F: Reduce + 'static,
    {
        // FIXME: Still don't get the correct approach to assign the
        // `reduce` to `self.reduce`  :-()
        self.reduce = Some(Box::pin(reduce));
    }

    fn execute<O>(&mut self) -> O {
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
    #[test]
    pub fn word_count() {
    }
}
