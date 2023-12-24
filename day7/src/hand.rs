use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for Card {}

#[derive(Eq, PartialEq)]
pub struct Hand {
    cards: [Card; 5],
    bet: u64,
    rank: u8,
}

fn compute_rank(cards: &[Card; 5]) -> u8 {
    let mut card_counts = HashMap::new();

    for item in cards {
        *card_counts.entry(item).or_insert(0) += 1;
    }

    match card_counts.values().max().unwrap() {
        5 => return 7,
        4 => return 6,
        3 => {
            return if card_counts.len() == 2 {
                // Full house
                5
            } else {
                // Three of a kind
                4
            };
        }
        2 => {
            return if card_counts.len() == 3 {
                // Two pairs
                3
            } else {
                // One pair
                2
            };
        }
        1 => return 1,
        _ => panic!("Invalid card counts"),
    }
}

impl Hand {
    pub fn new(cards: &str, bet: u64) -> Self {
        if cards.len() != 5 {
            panic!("Each hand should have exactly 5 cards. Got {}", cards);
        }

        let mut cards_iter = cards.chars().filter_map(|c| match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        });

        let cards = [
            cards_iter.next().unwrap(),
            cards_iter.next().unwrap(),
            cards_iter.next().unwrap(),
            cards_iter.next().unwrap(),
            cards_iter.next().unwrap(),
        ];

        Hand {
            bet,
            rank: compute_rank(&cards),
            cards,
        }
    }

    pub fn bet(&self) -> u64 {
        self.bet
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| a.cmp(b))
                .find(|&order| order != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            order => order,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
