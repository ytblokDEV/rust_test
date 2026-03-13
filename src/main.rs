use rand::prelude::*;

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

    // println!("{:?}", deck); chyba dobrze pomieszane
    println!("{:?}", deck.len());

    if let Some(random_card) = deck.choose(&mut rng) {
        println!("{:?}", random_card);
    }

    for i in 0..=51 {
        let card = &deck[i];
        println!("{:?}", card);
        println!("{}", i + 1);
    } //numer iteracjii
    //cały deck w osobnych linijkach
}
