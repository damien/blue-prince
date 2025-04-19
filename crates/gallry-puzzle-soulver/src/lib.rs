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
    words: Option<Vec<String>>,
}

impl WordGenerator {
    fn new(slots: Vec<Slot>) -> Self {
        Self { slots, words: None }
    }

    // Generate words using the current slot values.
    fn generate(&mut self) {
        let mut words = Vec::new();
        let mut indices = vec![0; self.slots.len()];
        
        while indices[0] < self.slots[0].options.len() {
            // Build current word
            let word: String = self.slots.iter()
                .zip(&indices)
                .map(|(slot, &i)| slot.options[i])
                .collect();
            words.push(word);
            
            // Increment indices
            for i in (0..indices.len()).rev() {
                indices[i] += 1;
                if indices[i] < self.slots[i].options.len() {
                    break;
                }
                if i > 0 {
                    indices[i] = 0;
                }
            }
        }
        
        self.words = Some(words);
    }
}

#[test]
fn test_generate() {
    let mut word_generator = WordGenerator::new(
        vec![
            Slot::new(vec!['c', 'b', 'r']),
            Slot::new(vec!['a', 'i', 'o']),
            Slot::new(vec!['t', 's', 'e']),
        ]
    );

    word_generator.generate();

    assert_eq!(word_generator.words, Some(vec![
        "cat".to_string(), "cas".to_string(), "cae".to_string(),
        "cit".to_string(), "cis".to_string(), "cie".to_string(),
        "cot".to_string(), "cos".to_string(), "coe".to_string(),
        "bat".to_string(), "bas".to_string(), "bae".to_string(),
        "bit".to_string(), "bis".to_string(), "bie".to_string(),
        "bot".to_string(), "bos".to_string(), "boe".to_string(),
        "rat".to_string(), "ras".to_string(), "rae".to_string(),
        "rit".to_string(), "ris".to_string(), "rie".to_string(),
        "rot".to_string(), "ros".to_string(), "roe".to_string()
    ]));
}
