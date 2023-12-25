use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
pub struct Card {
    pub hand: String,
    pub ordered_hand: String,
    pub bid: i32,
    pub value: HandType,
}
impl Card {
    pub fn read_card(line: &str) -> Self {
        let mut line = line.split(" ");
        let (hand, bid) = (
            line.next().unwrap().to_string(),
            line.next()
                .unwrap()
                .parse::<i32>()
                .expect("Failed to parse string"),
        );
        let value = Card::compute_value(&hand);
        let ordered_hand = Card::compute_ordered_hand(&hand);
        Self {
            hand,
            ordered_hand,
            bid,
            value,
        }
    }

    pub fn compute_value(hand: &str) -> HandType {
        let hand = Self::replace_joker(hand);
        let mut set = HashSet::new();
        for c in hand.chars() {
            set.insert(c);
        }
        match set.len() {
            1 => return HandType::FiveOfAKind,
            4 => return HandType::OnePair,
            5 => return HandType::HighCard,
            _ => (),
        }
        if set.len() == 2 {
            if hand
                .chars()
                .filter(|x| x == &hand.chars().next().unwrap())
                .count()
                == 1
                || hand
                    .chars()
                    .filter(|x| x == &hand.chars().next().unwrap())
                    .count()
                    == 4
            {
                return HandType::FourOfAKind;
            }
            return HandType::FullHouse;
        }
        let mut map = HashMap::new();
        hand.chars().for_each(|x| {
            match map.get(&x) {
                None => map.insert(x, 1),
                Some(count) => map.insert(x, count + 1),
            };
        });
        for v in map.values() {
            if v == &3 {
                return HandType::ThreeOfAKind;
            }
        }
        HandType::TwoPair
    }

    pub fn replace_joker(hand: &str) -> String {
        let mut aux_map = HashMap::new();
        for c in hand.chars() {
            match aux_map.get(&c) {
                None => aux_map.insert(c, 1),
                Some(count) => aux_map.insert(c, count + 1),
            };
        }
        let mut maxi = (0, 'x');
        aux_map.remove(&'J');
        for (k, v) in aux_map.iter() {
            if v > &maxi.0 {
                maxi = (*v, *k);
            }
        }
        hand.chars()
            .map(|x| {
                if x == 'J' {
                    return maxi.1;
                }
                x
            })
            .collect()
    }

    pub fn compute_ordered_hand(hand: &str) -> String {
        hand.chars()
            .map(|x| match x {
                '2' => 'a',
                '3' => 'b',
                '4' => 'c',
                '5' => 'd',
                '6' => 'e',
                '7' => 'f',
                '8' => 'g',
                '9' => 'h',
                'T' => 'i',
                'J' => 'Z',
                'Q' => 'k',
                'K' => 'l',
                'A' => 'm',
                _ => panic!("Failed to compute ordered hand"),
            })
            .collect()
    }
}
