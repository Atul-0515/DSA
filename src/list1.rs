
pub fn run() -> i32 {
    8
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(), 8);
    }
}