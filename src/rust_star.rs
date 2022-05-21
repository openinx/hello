pub fn test_start() {
    let mut num: i32 = 3;

    println!("Before Edit: {}", num);

    edit(&mut num);

    println!("After Edit: {}", num);
}

fn edit(num: &mut i32) {
    // Deference the num so that you can assign to the place in memory it points to.
    *num = 4;
}

#[cfg(test)]
mod tests {
    use crate::rust_star::edit;

    #[test]
    pub fn basics() {
        let mut num = 3;

        assert_eq!(3, num);
        edit(&mut num);
        assert_eq!(4, num);
    }
}
