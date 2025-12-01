pub fn day1(input: String) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example() {
        // Given
        let input = String::from("L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82");

        // When
        let result = day1(input);

        // Then
        assert_eq!(result, 3);
    }
}
