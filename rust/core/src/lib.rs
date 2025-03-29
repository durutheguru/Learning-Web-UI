pub fn add(left: usize, right: usize) -> usize {
    println!("Adding numbers {}, {}", left, right);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
