pub mod foo;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::foo::foo;

    #[test]
    fn it_works() {
        assert_eq!(10, foo::sum_ten(0));
    }

    #[test]
    fn it_works_too() {
        assert_eq!(12, foo::sum_ten(2));
    }

    #[test]
    fn it_works_too_too() {
        assert_eq!(14, foo::sum_ten(4));
    }
}
