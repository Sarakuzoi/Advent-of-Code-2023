use std::{fs::File, io::Read};

use day_seven_b::{Card, HandType};

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut cards = Vec::new();
    for line in contents.lines() {
        cards.push(Card::read_card(line));
    }

    let mut v = ["abc", "abb", "zzz", "Zaa", "aZc", "ZZZ"];
    v.sort();
    let mut s = 0;

    let mut high_card = make_sortable(&cards, HandType::HighCard);
    let mut one_pair = make_sortable(&cards, HandType::OnePair);
    let mut two_pair = make_sortable(&cards, HandType::TwoPair);
    let mut three_of_a_kind = make_sortable(&cards, HandType::ThreeOfAKind);
    let mut full_house = make_sortable(&cards, HandType::FullHouse);
    let mut four_of_a_kind = make_sortable(&cards, HandType::FourOfAKind);
    let mut five_of_a_kind = make_sortable(&cards, HandType::FiveOfAKind);
    high_card.sort();
    one_pair.sort();
    two_pair.sort();
    three_of_a_kind.sort();
    full_house.sort();
    four_of_a_kind.sort();
    five_of_a_kind.sort();

    let mut cnt = 0;
    let aux = [
        high_card,
        one_pair,
        two_pair,
        three_of_a_kind,
        full_house,
        four_of_a_kind,
        five_of_a_kind,
    ];
    for v in aux {
        v.iter().for_each(|x| {
            cnt += 1;
            s += cnt * x.1;
        });
    }
    println!("{s}");
    Ok(())
}

fn make_sortable(cards: &Vec<Card>, value: HandType) -> Vec<(&str, i32)> {
    cards
        .iter()
        .filter(|x| x.value == value)
        .map(|x| (x.ordered_hand.as_str(), x.bid))
        .collect()
}
