//! Gallary Puzzle Soulver
//! 
//! Each gallery item has a corresponding word of arbitrary length.
//! Each letter in the word has it's own distinct set of 8 possible characters.
//! Produce a list of all the words that can be formed from the letters for a given list of characters and their corresponding possible characters.

use std::ops::Deref;

// Define a slot as a character with a list of possible characters and a selected character.
//
// Dereferencing the slot returns the selected character.
struct Slot {
    options: Vec<char>,
    current: usize, // index of the selected character
}

impl Slot {
    fn new(options: Vec<char>) -> Self {
        Self { options, current: 0 }
    }
}

impl Into<String> for Slot {
    fn into(self) -> String {
        self.options[self.current].to_string()
    }
}

#[test]
fn test_to_string() {
    let slot = Slot::new(vec!['a', 'b', 'c']);
    assert_eq!(slot.to_string(), "a");
}

impl Deref for Slot {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.options[self.current]
    }
}

#[test]
fn test_deref() {
    let slot = Slot::new(vec!['a', 'b', 'c']);
    assert_eq!(*slot, 'a');
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

#[test]
fn test_iterator() {
    let slot = Slot::new(vec!['a', 'b', 'c']);
    assert_eq!(slot.collect::<Vec<_>>(), vec!['a', 'b', 'c']);
}

struct WordGenerator {
    slots: Vec<Slot>,
    current: usize, // index of the selected slot
    words: Option<Vec<String>>,
}

impl WordGenerator {
    fn new(slots: Vec<Slot>) -> Self {
        Self { slots, current: 0, words: None }
    }

    // Generate words using the current slot values.
    fn generate(&mut self) {
    }
}

#[test]
fn test_generate() {
    let mut word_generator = WordGenerator::new(
        vec![
            Slot::new(vec!['a', 'b', 'c']),
            Slot::new(vec!['a', 'b', 'c'])
        ]
    );

    word_generator.generate();

    assert_eq!(word_generator.words, Some(vec![
        "aa".to_string(), "ab".to_string(), "ac".to_string(),
        "ba".to_string(), "bb".to_string(), "bc".to_string(), 
        "ca".to_string(), "cb".to_string(), "cc".to_string()
    ]));
}
