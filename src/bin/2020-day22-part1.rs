use std::collections::VecDeque;

fn main() -> Result<(), String> {
    let input = include_str!("../../input/2020-day22.txt");

    let mut player_inputs = input.split("\n\n");

    let player1_input = player_inputs.next().ok_or("Missing player 1 input".to_string())?;
    let player2_input = player_inputs.next().ok_or("Missing player 2 input".to_string())?;

    let mut player1_deck = parse_deck(player1_input)?;
    let mut player2_deck = parse_deck(player2_input)?;

    play(&mut player1_deck, &mut player2_deck);

    println!("{}", std::cmp::max(player1_deck.score(), player2_deck.score()));

    Ok(())
}

fn play(deck1: &mut Deck, deck2: &mut Deck) {
    loop {
        if deck1.is_empty() || deck2.is_empty() {
            return;
        }

        let card1 = deck1.draw().unwrap();
        let card2 = deck2.draw().unwrap();

        if card1 > card2 {
            deck1.pick_up(card1);
            deck1.pick_up(card2);
        } else if card2 > card1 {
            deck2.pick_up(card2);
            deck2.pick_up(card1);
        } else {
            panic!("Undefined behavior: the two drawed cards have the same value: {} vs. {}", card1, card2);
        }
    }
}

fn parse_deck(player_input: &str) -> Result<Deck, String>
{
    let mut lines = player_input.lines();

    lines.next().ok_or(format!("Empty player input: {:?}", player_input))?;

    let cards = lines
        .map(|line| line.parse().map_err(|error| format!("Could not parse line into card number: {:?}", error)))
        .collect::<Result<_, _>>()?;

    Ok(Deck { cards })
}

#[derive(Debug)]
struct Deck {
    cards: VecDeque<u64>,
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn draw(&mut self) -> Option<u64> {
        self.cards.pop_front()
    }

    fn pick_up(&mut self, card: u64) {
        self.cards.push_back(card);
    }

    fn score(&self) -> u64 {
        self.cards.iter()
            .rev()
            .enumerate()
            .map(|(index, card)| (index as u64 + 1) * card)
            .sum()
    }
}
