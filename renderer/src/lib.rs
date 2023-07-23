pub fn test() {
    println!("test function");
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        test();
    }
}
