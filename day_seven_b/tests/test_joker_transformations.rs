#[cfg(test)]
mod test_joker_transformations {

    use day_seven_b::{Card, HandType};

    #[test]
    fn test_ordered_transformation() {
        let x = Card::read_card("J54T4 977");
        assert_eq!("Zdcic".to_string(), x.ordered_hand);
    }

    #[test]
    fn test_joker_three_of_a_kind() {
        assert_eq!(HandType::ThreeOfAKind, Card::compute_value("J54T4"));
    }

    #[test]
    fn test_replace_joker() {
        assert_eq!("454T4".to_string(), Card::replace_joker("J54T4"));
    }
}
