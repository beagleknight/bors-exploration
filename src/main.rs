pub mod foo;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::foo::foo;

    #[test]
    fn it_works() {
        assert_eq!(10, foo::get_value(0));
    }

    #[test]
    fn it_works_too() {
        assert_eq!(12, foo::get_value(2));
    }
}
