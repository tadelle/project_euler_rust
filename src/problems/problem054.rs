//! Poker hands
//!
//! In the card game poker, a hand consists of
//! five cards and are ranked, from lowest to
//! highest, in the following way:
//!
//! High Card: Highest value card.
//! One Pair: Two cards of the same value.
//! Two Pairs: Two different pairs.
//! Three of a Kind: Three cards of the same value.
//! Straight: All cards are consecutive values.
//! Flush: All cards of the same suit.
//! Full House: Three of a kind and a pair.
//! Four of a Kind: Four cards of the same value.
//! Straight Flush: All cards are consecutive values of same suit.
//! Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
//! The cards are valued in the order:
//! 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
//!
//! If two players have the same ranked hands
//! then the rank made up of the highest value wins;
//! for example, a pair of eights beats a pair of fives
//! (see example 1 below). But if two ranks tie,
//! for example, both players have a pair of queens,
//! then highest cards in each hand are compared
//! (see example 4 below); if the highest cards tie
//! then the next highest cards are compared, and so on.
//!
//! Consider the following five hands dealt to two players:
//!
//! Hand  Player 1            Player 2             Winner
//! 1     5H 5C 6S 7S KD      2C 3S 8S 8D TD
//!       Pair of Fives       Pair of Eights       Player 2
//!
//! 2     5D 8C 9S JS AC      2C 5C 7D 8S QH
//!       Highest card Ace    Highest card Queen   Player 1
//!
//! 3     2D 9C AS AH AC      3D 6D 7D TD QD
//!       Three Aces          Flush with Diamonds  Player 2
//!
//! 4     4D 6S 9H QH QC      3D 6D 7H QD QS
//!       Pair of Queens      Pair of Queens
//!       Highest card Nine   Highest card Seven   Player 1
//!
//! 5     2H 2D 4C 4D 4S      3C 3D 3S 9S 9D
//!       Full House          Full House
//!       With Three Fours    With Three Threes    Player 1
//!
//! The file, poker.txt, contains one-thousand
//! random hands dealt to two players. Each line of
//! the file contains ten cards (separated by a
//! single space): the first five are Player 1's cards
//! and the last five are Player 2's cards.
//! You can assume that all hands are valid
//! (no invalid characters or repeated cards),
//! each player's hand is in no specific order,
//! and in each hand there is a clear winner.
//!
//! How many hands does Player 1 win?
use std::{cmp::Ordering, fs};
use super::Problem;

crate::base_problem!(376, "Poker hands");

#[derive(Debug, Clone)]
struct Card {
    value: u8,
    suit: char
}

impl Card {
    fn new(card: String) -> Card {

        let value: u8 = match card.chars().next().unwrap_or('0') {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0
        };

        let suit: char = match card.chars().nth(1).unwrap_or('0') {
            'C' => '♥',
            'H' => '♣',
            'D' => '♦',
            'S' => '♠',
            _ => ' '
        };

        Card { value, suit }
    }
}

struct HandCard {
    hand: Vec<Card>,
    rank_type: Rank,
    rank_value: u8,
}

impl PartialEq for HandCard {
    fn eq(&self, other: &Self) -> bool {
        let vec1: Vec<u8> = self.hand.clone().into_iter().map(|card| card.value).collect();
        let vec2: Vec<u8> = other.hand.clone().into_iter().map(|card| card.value).collect();

        for card in vec1 {
            if !vec2.contains(&card) {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for HandCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank_value != other.rank_value {
            Some(self.rank_value.cmp(&other.rank_value))
        } else {
            match (self.rank_type, other.rank_type) {
                (Rank::StraightFlush(v1), Rank::StraightFlush(v2)) 
                    => Some(v1.cmp(&v2)),
                (Rank::HighCard(v1), Rank::HighCard(v2)) => Some(v1.cmp(&v2)),
                (Rank::OnePair { pair_card: p1, high_card: h1 }, Rank::OnePair { pair_card: p2, high_card: h2 }) => {
                    if p1 != p2{
                        Some(p1.cmp(&p2))
                    } else {
                        Some(h1.cmp(&h2))
                    }
                },
                (Rank::TwoPairs { pair_card1: p11, pair_card2: p21, high_card: h1 }, Rank::TwoPairs { pair_card1: p12, pair_card2: p22, high_card:h2 }) => {
                    if p11 != p12 {
                        Some(p11.cmp(&p12))
                    } else if p21 != p22 {
                        Some(p21.cmp(&p22))
                    } else {
                        Some(h1.cmp(&h2))
                    }
                },
                (Rank::ThreeOfAKind(v1), Rank::ThreeOfAKind(v2)) => Some(v1.cmp(&v2)),
                (Rank::Straight(v1), Rank::Straight(v2)) => Some(v1.cmp(&v2)),
                (Rank::Flush(v1), Rank::Flush(v2)) => Some(v1.cmp(&v2)),
                (Rank::FullHouse(v1), Rank::FullHouse(v2)) => Some(v1.cmp(&v2)),
                _ => None,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rank {
    HighCard(u8),
    OnePair{pair_card: u8, high_card: u8},
    TwoPairs{pair_card1: u8, pair_card2: u8, high_card: u8},
    ThreeOfAKind(u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8),
    FourOfAKind(u8),
    StraightFlush(u8),
    RoyalFlush
}

impl HandCard {
    fn new(card1: Card, card2: Card, card3: Card, card4: Card, card5: Card) -> HandCard {
        let cards: Vec<Card> = vec!(card1, card2, card3, card4, card5);
        let mut values: Vec<u8> = cards.clone().into_iter().map(|c| c.value).collect();
        let suits: Vec<char> = cards.clone().into_iter().map(|c| c.suit).collect();

        values.sort();

        let length = cards.len();
        let rank: Rank;
        let mut max: u8 = *values.last().unwrap();
        let mut is_same_suit = true;
        let mut is_consecutive_values = true;
        let value: u8;

        for index in 1..length {
            if suits[0] != suits[index] {
                is_same_suit = false;
                break;
            }
        }

        for index in 1..length {
            if values[index - 1] + 1 != values[index] {
                is_consecutive_values = false;
                break;
            }
        }

        let mut is_pair = false;
        let mut is_two_pair = false;
        let mut is_triple = false;
        let mut is_four = false;

        let mut card_pair1 = 0;
        let mut card_pair2 = 0;
        let mut card_remainder = 0;

        let mut exc_value = 0;
        for idx1 in (0..(length - 1)).rev() {
            let count_values = values.iter().filter(|v| *v == &values[idx1] && *v != &exc_value).count();

            match count_values {
                4 => {
                    is_four = true;
                    max = values[idx1];
                },
                3 => {
                    is_triple = true;
                    exc_value = values[idx1];
                    max = values[idx1];
                },
                2 => {
                    if is_pair {
                        is_two_pair = true;
                        card_pair2 = values[idx1];
                    } else {
                        is_pair = true;
                        card_pair1 = values[idx1];
                        exc_value = values[idx1];
                    }
                },
                _ => {
                    if values[idx1] > card_remainder {
                        card_remainder = values[idx1];
                    }
                }
            }
        }

        if is_consecutive_values {
            if is_same_suit {
                if max == 14 {
                    rank = Rank::RoyalFlush;
                    value = 10;
                } else {
                    rank = Rank::StraightFlush(max);
                    value = 9;
                }
            } else {
                rank = Rank::Straight(max);
                value = 5;
            }
        } else if is_same_suit {
            rank = Rank::Flush(max);
            value = 6;
        } else if is_four {
            rank = Rank::FourOfAKind(max);
            value = 8;
        } else if is_triple {
            if is_pair {
                rank = Rank::FullHouse(max);
                value = 9;
            } else {
                rank = Rank::ThreeOfAKind(max);
                value = 4;
            }
        } else if is_two_pair {
            let high_card = values.iter().filter(|c| *c != &card_pair1 && *c != &card_pair2).rev().last().unwrap_or(&0u8);
            rank = Rank::TwoPairs { pair_card1: card_pair1, pair_card2: card_pair2, high_card: *high_card };
            value = 3;
        } else if is_pair {
            let high_card = values.iter().filter(|c| *c != &card_pair1).max().unwrap_or(&0u8);
            rank = Rank::OnePair { pair_card: card_pair1, high_card: *high_card };
            value = 2;
        } else {
            rank = Rank::HighCard(max);
            value = 1;
        }
        HandCard { hand: cards, rank_type: rank, rank_value: value }
    }
}

fn get_result_problem() -> i64 {
    let text = fs::read_to_string("src/data/p054_poker.txt").unwrap_or(String::from(""));
    let lines = text.lines();
    let mut player1_win = 0;

    for line in lines {
        let cards: Vec<_> = line.split(' ').collect();

        let player1_handcard = HandCard::new(
            Card::new(cards[0].to_string()),
            Card::new(cards[1].to_string()), 
            Card::new(cards[2].to_string()),
            Card::new(cards[3].to_string()),
            Card::new(cards[4].to_string())
        );

        let player2_handcard = HandCard::new(
            Card::new(cards[5].to_string()),
            Card::new(cards[6].to_string()), 
            Card::new(cards[7].to_string()),
            Card::new(cards[8].to_string()),
            Card::new(cards[9].to_string())
        );

        if player1_handcard > player2_handcard {
            player1_win += 1;
        }
    }

    player1_win
}
