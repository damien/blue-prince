//! Gallary Puzzle Soulver
//! 
//! Each gallery item has a corresponding word of arbitrary length.
//! Each letter in the word has it's own distinct set of 8 possible characters.
//! Produce a list of all the words that can be formed from the letters for a given list of characters and their corresponding possible characters.

use std::ops::Deref;

// Define a slot as a character with a list of possible characters and a selected character.
//
// Dereferencing the slot returns the selected character.
pub struct Slot {
    options: Vec<char>,
    current: usize, // index of the selected character
}

impl Slot {
    pub fn new(options: Vec<char>) -> Self {
        Self { options, current: 0 }
    }
}

impl Into<String> for Slot {
    fn into(self) -> String {
        self.options[self.current].to_string()
    }
}

impl Deref for Slot {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.options[self.current]
    }
}

impl Iterator for Slot {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.options.len() {
            let result = Some(self.options[self.current]);
            self.current += 1;
            result
        } else {
            None
        }
    }
}

pub struct WordGenerator {
    slots: Vec<Slot>,
    words: Option<Vec<String>>,
}

impl WordGenerator {
    pub fn new(slots: Vec<Slot>) -> Self {
        Self { slots, words: None }
    }

    // Generate words using the current slot values.
    pub fn generate(&mut self) {
        self.words = Some(
            self.slots.iter()
                .map(|slot| slot.options.iter())
                .fold(vec![String::new()], |acc, options| {
                    acc.iter()
                        .flat_map(|prefix| options.clone().map(move |&c| format!("{}{}", prefix, c)))
                        .collect()
                })
        );
    }
    
    pub fn get_words(&self) -> Option<&Vec<String>> {
        self.words.as_ref()
    }
}
