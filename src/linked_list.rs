// Linked List Implementation
fn hello() -> String {
    format!("Hello World")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(hello(), "Hello World".to_string());
    }
}