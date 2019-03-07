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
        .into_iter()
        .chain(consonants.into_iter())
        .cloned()
    // randomizes the order
        .choose_multiple_fill(&mut rng, &mut out[..]);
    out
}

pub struct Game {
    pub input: String,
    pub letters: [char; 7],
    pub words: BTreeSet<String>,
    pub dict: BTreeSet<String>,
    pub score: usize,
    pub error: Option<String>,
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

    fn score(&self) -> usize {
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
            let score = self.score();
            if self.words.insert(replace(&mut self.input, String::new())) {
                self.score += score;
            } else {
                self.error = Some("You already found that word!".into());
            }
        }
    }
}