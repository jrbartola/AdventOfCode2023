use std::collections::HashSet;

pub struct ScratchCard {
    winners: HashSet<u32>,
    drawn: HashSet<u32>,
    num_winners: u32,
}

impl ScratchCard {
    pub fn new(winners: HashSet<u32>, drawn: HashSet<u32>) -> Self {
        let num_winners = (&winners).intersection(&drawn).collect::<Vec<_>>().len() as u32;

        ScratchCard {
            winners,
            drawn,
            num_winners,
        }
    }

    pub fn get_value(&self) -> u32 {
        let drawn_winners = self.get_num_winners();

        if drawn_winners > 0 {
            return 2_i32.pow(drawn_winners - 1) as u32;
        }

        0
    }

    fn get_num_winners(&self) -> u32 {
        self.num_winners
    }
}

pub struct ScratchCardBoard {
    cards: Vec<ScratchCard>,
}

impl ScratchCardBoard {
    pub fn new(cards: Vec<ScratchCard>) -> Self {
        ScratchCardBoard { cards }
    }

    // Compute the total number of scratchcards won (part 2)
    // https://adventofcode.com/2023/day/4#part2
    pub fn compute_winners(&self) -> u32 {
        let mut card_counts = vec![1; self.cards.len()];

        for (i, card) in self.cards.iter().enumerate() {
            println!("Card num: {}; Card counts: {}", i + 1, card_counts[i]);
            for _ in 0..card_counts[i] {
                for j in 1..=card.get_num_winners() as usize {
                    if i + j < card_counts.len() {
                        card_counts[i + j] += 1;
                    }
                }
            }
        }

        card_counts.iter().sum()
    }
}
