fn main() {
    println!("Hello, world!");
}

mod foo {
    pub fn get_value() -> i32 {
        return 10;
    }
}

#[cfg(test)]
mod tests {
    use super::foo;

    #[test]
    fn it_works() {
        assert_eq!(10, foo::get_value());
    }
}
