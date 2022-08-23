pub fn main(){
    println!("hello world");
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    pub fn test(){
        main();
    }
}