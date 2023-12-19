#[cfg(test)]
mod test {
    use day_seven_a::Card;

    #[test]
    fn test_numbers() {
        assert_eq!("abbba".to_string(), Card::compute_ordered_hand("23332"));
    }

    #[test]
    fn test_letters() {
        assert_eq!("mickl".to_string(), Card::compute_ordered_hand("AT4QK"));
    }
}
