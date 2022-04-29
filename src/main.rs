fn main() {
    println!("Hello, world!");
}

fn foo() -> i32 {
    return 4;
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, foo());
    }
}
