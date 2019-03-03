extern crate rand;

use crate::cards::{AnswerCard, QuestionCard};

struct Hand {
    pub player_id: usize,
    cards: Vec<AnswerCard>,
    in_play: Vec<AnswerCard>,
    pub points: usize,
}

impl Hand {
    fn count(&self) -> usize {
        self.cards.len()
    }

    fn gain_card(&mut self, card: AnswerCard) {
        self.cards.push(card)
    }

    fn remove_card(&mut self, card_id: usize) -> Option<AnswerCard> {
        for i in 0..self.cards.len() {
            if self.cards[i].id == card_id {
                return Some(self.cards.remove(i));
            }
        }
        None
    }

    fn play_card(&mut self, card_id: usize) {
        if let Some(card) = self.remove_card(card_id) {
            self.in_play.push(card)
        }
    }

    fn remove_in_play(&mut self) -> Vec<AnswerCard> {
        self.in_play.drain(0..).collect()
    }
}

struct Deck<T>
where
    T: Clone,
{
    deck: Vec<T>,
    discard_pile: Vec<T>,
}

use std::iter::FromIterator;
impl<T> FromIterator<T> for Deck<T>
where
    T: Clone,
{
    fn from_iter<I>(iter: I) -> Deck<T>
    where
        I: IntoIterator<Item = T>,
    {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut deck = Vec::<T>::from_iter(iter);
        deck.shuffle(&mut thread_rng());

        Deck {
            deck,
            discard_pile: Vec::new(),
        }
    }
}

impl<T> Deck<T>
where
    T: Clone,
{
    fn draw_only(&mut self) -> Option<T> {
        self.deck.pop()
    }

    fn draw(&mut self) -> Option<T> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        if let Some(card) = self.deck.pop() {
            Some(card)
        } else if !self.discard_pile.is_empty() {
            self.deck.append(&mut self.discard_pile);
            self.deck.shuffle(&mut thread_rng());
            return self.deck.pop();
        } else {
            None
        }
    }

    fn discard(&mut self, item: T) {
        self.discard_pile.push(item)
    }
}

type QuestionDeck = Deck<QuestionCard>;
type AnswerDeck = Deck<AnswerCard>;

struct Game {
    id: usize,
    current_question: QuestionCard,
    hands: Vec<Hand>,
    questions: QuestionDeck,
    answers: AnswerDeck,
}

impl Game {
    fn player_discard(&mut self, player_id: usize, card_id: usize) {
        for i in 0..self.hands.len() {
            if self.hands[i].player_id == player_id {
                if let Some(card) = self.hands[i].remove_card(card_id) {
                    self.answers.discard(card);
                    return;
                }
            }
        }
    }

    fn player_draw(&mut self, player_id: usize) {
        for i in 0..self.hands.len() {
            if self.hands[i].player_id == player_id {
                let card = self.answers.draw().unwrap();
                self.hands[i].gain_card(card);
                return;
            }
        }
    }

    fn discard_in_play(&mut self) {
        for hand in &mut self.hands {
            for card in hand.remove_in_play() {
                self.answers.discard(card);
            }
        }
    }

    fn next_question(&mut self) {
        use std::mem::replace;

        let new_question = self.questions.draw().unwrap();
        let discarded = replace(&mut self.current_question, new_question);
        self.questions.discard(discarded);
    }
}
