// --- Day 7: Camel Cards ---
// Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.
//
// "Did you bring the parts?"
//
// You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.
//
// "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.
//
// "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.
//
// After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.
//
// You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.
//
// Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.
//
// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.
//
// Every hand is exactly one type. From strongest to weakest, they are:
//
// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456
// Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.
//
// If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.
//
// So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).
//
// To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:
//
// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
// This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.
//
// So, the first step is to put the hands in order of strength:
//
// 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
// KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
// T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
// Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.
//
// Find the rank of every hand in your set. What are the total winnings?
//

use std::{cmp::Ordering, collections::HashMap, usize};

#[repr(u8)]
#[derive(Debug)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn get_hand_type(hand: &str) -> Option<HandType> {
    if hand.len() != 5 {
        return None;
    }

    let mut chars: HashMap<String, u32> = HashMap::new();
    hand.chars().for_each(|char| {
        chars.insert(
            char.to_string(),
            chars.get(&char.to_string()).unwrap_or(&0) + 1,
        );
    });

    if chars.len() == 5 {
        return Some(HandType::HighCard);
    } else if chars.len() == 4 {
        return Some(HandType::OnePair);
    } else if chars.len() == 3 && chars.values().any(|v| *v == 2) {
        return Some(HandType::TwoPair);
    } else if chars.len() == 3 && chars.values().any(|v| *v == 3) {
        return Some(HandType::ThreeOfAKind);
    } else if chars.len() == 2 && chars.values().any(|v| *v == 3) {
        return Some(HandType::FullHouse);
    } else if chars.len() == 2 && chars.values().any(|v| *v == 4) {
        return Some(HandType::FourOfAKind);
    } else if chars.len() == 1 {
        return Some(HandType::FiveOfAKind);
    } else {
        return None;
    }
}

pub fn total_winnings(data: &[&str]) -> Option<u32> {
    if data.is_empty() {
        return None;
    }

    let card_map: HashMap<&str, usize> = HashMap::from_iter(
        vec![
            "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
        ]
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, s)| (*s, idx)),
    );

    let mut hands = data
        .iter()
        .map(|line| {
            let biding = line.split_ascii_whitespace().collect::<Vec<_>>();
            let [hand, bid] = biding.as_slice() else {
                todo!()
            };
            let hand_type = get_hand_type(hand).unwrap() as u32;
            (hand_type, *hand, *bid)
        })
        .collect::<Vec<(u32, &str, &str)>>();

    // sort and reverse the hands
    hands.sort_by(|a, b| {
        let mut ord = a.0.cmp(&b.0);

        let mut idx = 0;
        while ord == Ordering::Equal && idx < a.1.len() {
            ord = card_map
                .get(a.1.get(idx..idx + 1).unwrap())
                .unwrap()
                .cmp(card_map.get(b.1.get(idx..idx + 1).unwrap()).unwrap());
            idx += 1;
        }

        ord
    });

    // rank and sum
    let total = hands.iter().enumerate().fold(0, |acc, tpl| {
        acc + (tpl.0 as u32 + 1) * (tpl.1).2.parse::<u32>().unwrap()
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use googletest::{assert_that, matchers::eq};

    use super::*;

    #[test]
    fn test_total_winnings() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let result = total_winnings(&input).unwrap();
        let expected = 6440;
        assert_that!(result, eq(expected));
    }

    #[test]
    fn test_get_hand_type() {
        let hand = "AAAAA";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FiveOfAKind as u32));

        let hand = "AAA8A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FourOfAKind as u32));

        let hand = "AA88A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FullHouse as u32));

        let hand = "AA28A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::ThreeOfAKind as u32));

        let hand = "2AA82";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::TwoPair as u32));

        let hand = "6878A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::OnePair as u32));

        let hand = "2358A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::HighCard as u32));
    }
}
