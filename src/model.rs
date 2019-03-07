use std::collections::BTreeSet;
use std::mem::replace;

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};

const VOWELS: &[char] = &['A', 'E', 'I', 'O', 'U', 'Y'];
const CONSONANTS: &[char] = &[
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Z',
];

fn pick_letters() -> [char; 7] {
    let mut rng = thread_rng();

    let num_vowels = rng.gen_range(1, 3);

    let vowels = VOWELS.choose_multiple(&mut rng, num_vowels);
    let consonants = CONSONANTS.choose_multiple(&mut rng, 7 - num_vowels);

    let mut out = ['\0'; 7];
    vowels
        .chain(consonants)
        .cloned()
        // randomizes the order
        .choose_multiple_fill(&mut rng, &mut out[..]);
    out
}

pub struct Game {
    input: String,
    letters: [char; 7],
    words: BTreeSet<String>,
    dict: BTreeSet<String>,
    score: usize,
    error: Option<String>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            letters: pick_letters(),
            words: BTreeSet::new(),
            dict: BTreeSet::new(),
            score: 0,
            error: None,
        }
    }

    pub fn restart(&mut self) {
        self.input.clear();
        self.letters = pick_letters();
        self.words.clear();
        self.score = 0;
    }

    pub fn set_dict(&mut self, dict: BTreeSet<String>) {
        self.dict = dict;
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn letters(&self) -> [char; 7] {
        self.letters
    }

    pub fn words(&self) -> impl Iterator<Item = &String> {
        self.words.iter()
    }

    pub fn error(&self) -> &Option<String> {
        &self.error
    }

    pub fn clear_error(&mut self) {
        self.error.take();
    }

    fn check(&self) -> Option<String> {
        let mut has_center = false;
        for c in self.input.chars() {
            if c == self.letters[0] {
                has_center = true;
            } else if !self.letters.contains(&c) {
                return Some(format!("{} is not in the letter set.", c));
            }
        }

        match self.input.len() {
            0 => return Some("No input entered.".into()),
            1..=3 => return Some("Words must be at least 4 characters.".into()),
            _ => (),
        }

        if !has_center {
            return Some(format!(
                "Words must include the center letter ({}).",
                self.letters[0]
            ));
        }

        if !self.dict.contains(&self.input.to_lowercase()) {
            return Some(format!("{} is not in the dictionary.", self.input));
        }

        None
    }

    fn eval_score(&self) -> usize {
        if self.letters.iter().all(|&c| self.input.contains(c)) {
            3
        } else {
            1
        }
    }

    pub fn submit(&mut self) {
        if let Some(err) = self.check() {
            self.error = Some(err);
            self.input.clear();
        } else {
            let score = self.eval_score();
            if self.words.insert(replace(&mut self.input, String::new())) {
                self.score += score;
            } else {
                self.error = Some("You already found that word!".into());
            }
        }
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }

    pub fn push(&mut self, c: char) {
        self.input.push(c);
    }
}
