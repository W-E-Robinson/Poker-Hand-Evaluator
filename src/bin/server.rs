fn main() {
    let result = String::from("hello");
    println!("{}", result);
}

fn helper() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(helper(), 42);
    }
}
