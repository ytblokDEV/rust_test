use rand::prelude::*;
use std::fmt;
use yansi::Paint;
#[derive(Debug, Copy, Clone)]
enum Kind {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(Debug, Copy, Clone)]
struct Card {
    color: Color,
    kind: Kind,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Diamonds => write!(f, "{}", "Diamonds ♦".red().bold()),
            Color::Hearts => write!(f, "{}", "Hearts ♥".red().bold()),
            Color::Spades => write!(f, "{}", "Spades ♠".bright_black().bold()),
            Color::Clubs => write!(f, "{}", "Clubs ♣".bright_black().bold()),
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card: {} of {}", self.kind, self.color)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let colors = [Color::Diamonds, Color::Hearts, Color::Spades, Color::Clubs];
    let kinds = [
        Kind::Ace,
        Kind::Two,
        Kind::Three,
        Kind::Four,
        Kind::Five,
        Kind::Six,
        Kind::Seven,
        Kind::Eight,
        Kind::Nine,
        Kind::Ten,
        Kind::Jack,
        Kind::Queen,
        Kind::King,
    ];

    let mut deck = vec![];
    for &color in &colors {
        for &kind in &kinds {
            deck.push(Card { color, kind });
        }
    }

    deck.shuffle(&mut rng);

    for i in 0..=51 {
        let card = &deck[i];
        println!("{}, place in deck: {}", card, i + 1);
    }
}
