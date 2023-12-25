#[cfg(test)]
mod test_ordered_hand {
    use day_seven_b::Card;

    #[test]
    fn test_numbers() {
        assert_eq!("abbba".to_string(), Card::compute_ordered_hand("23332"));
    }

    #[test]
    fn test_letters() {
        assert_eq!("mickl".to_string(), Card::compute_ordered_hand("AT4QK"));
    }
}
