#[cfg(test)]
mod test_hand_type {
    use day_seven_b::{Card, HandType};

    #[test]
    fn five_of_a_kind() {
        let hand = "AAAAA";
        assert_eq!(Card::compute_value(hand), HandType::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind() {
        let hand = "AA8AA";
        assert_eq!(Card::compute_value(hand), HandType::FourOfAKind);
    }

    #[test]
    fn full_house() {
        let hand = "23332";
        assert_eq!(Card::compute_value(hand), HandType::FullHouse);
    }

    #[test]
    fn three_of_a_kind() {
        let hand = "TTT98";
        assert_eq!(Card::compute_value(hand), HandType::ThreeOfAKind);
    }

    #[test]
    fn two_pair() {
        let hand = "23432";
        assert_eq!(Card::compute_value(hand), HandType::TwoPair);
    }

    #[test]
    fn one_pair() {
        let hand = "A23A4";
        assert_eq!(Card::compute_value(hand), HandType::OnePair);
    }

    #[test]
    fn high_card() {
        let hand = "23456";
        assert_eq!(Card::compute_value(hand), HandType::HighCard);
    }
}
