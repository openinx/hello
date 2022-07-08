use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn basic(){
        // i is owned and contains no references, thus it's 'static:
        let i = 5;
        print_it(i);

        // oops, &i only has the lifetime defined by the scope of
        // basic(), so it's not 'static:
        // print_it(&i); <-- compile error.
    }
}