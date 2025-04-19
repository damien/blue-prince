//! # Gallary Puzzle Soulver
//!
//! This library helps solve word puzzles where each character position has a limited set of possible options.
//!
//! ## Overview
//!
//! Each gallery item has a corresponding word of arbitrary length.
//! Each letter in the word has its own distinct set of possible characters.
//! This library helps produce a list of all possible words that can be formed,
//! and optionally filters them against a dictionary of valid words.
//!
//! ## Features
//!
//! - Generate all possible word combinations from sets of character options
//! - Filter generated words against an embedded word list
//! - Support for custom word lists
//! - Efficient HashSet-based lookups for word filtering
//!
//! ## Example
//!
//! ```
//! use gallry_puzzle_soulver::{Slot, WordGenerator};
//!
//! // Create slots with possible characters for each position
//! let slots = vec![
//!     Slot::new(vec!['c', 'b']),
//!     Slot::new(vec!['a', 'o']),
//!     Slot::new(vec!['t', 'r']),
//! ];
//!
//! // Create a generator with the embedded word list
//! let generator = WordGenerator::with_slots(slots);
//!
//! // Get words that exist in the word list
//! for word in generator.iter() {
//!     println!("Valid word: {}", word);
//! }
//! ```

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::ops::Deref;

// Embed the wordlist at compile time
const EMBEDDED_WORDLIST: &str = include_str!("../data/words.txt");

/// A character position with multiple possible character options.
///
/// Each `Slot` represents a single position in a word, with a set of possible characters
/// that could appear in that position. It also implements `Iterator` to allow iterating
/// through all possible characters in the slot.
///
/// # Examples
///
/// ```
/// use gallry_puzzle_soulver::Slot;
///
/// // Create a slot with three possible characters
/// let slot = Slot::new(vec!['a', 'b', 'c']);
///
/// // Use it as a character via deref coercion (defaults to first option)
/// assert_eq!(*slot, 'a');
///
/// // Convert to string
/// assert_eq!(slot.to_string(), "a");
///
/// // Iterate through all options
/// let chars: Vec<char> = slot.collect();
/// assert_eq!(chars, vec!['a', 'b', 'c']);
/// ```
#[derive(Clone, Debug)]
pub struct Slot {
    /// All possible characters for this position
    options: Vec<char>,
    /// Current index when iterating
    current: usize,
}

impl Slot {
    /// Creates a new Slot with the given character options.
    ///
    /// # Parameters
    ///
    /// * `options` - A vector of possible characters for this position
    ///
    /// # Examples
    ///
    /// ```
    /// use gallry_puzzle_soulver::Slot;
    ///
    /// let slot = Slot::new(vec!['a', 'b', 'c']);
    /// ```
    pub fn new(options: Vec<char>) -> Self {
        Self {
            options,
            current: 0,
        }
    }
}

impl From<Slot> for String {
    /// Converts the slot to a String, using the currently selected character.
    fn from(val: Slot) -> Self {
        val.options[val.current].to_string()
    }
}

impl Deref for Slot {
    type Target = char;

    /// Dereferences to the currently selected character.
    /// By default, this is the first character in the options list.
    fn deref(&self) -> &Self::Target {
        &self.options[self.current]
    }
}

impl Iterator for Slot {
    type Item = char;

    /// Iterates through all possible characters in this slot.
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

/// A generator for creating and filtering possible words based on character options.
///
/// The `WordGenerator` combines multiple `Slot`s to generate all possible word combinations.
/// It can filter these words against a word list to find valid words.
///
/// # Examples
///
/// Basic usage with the default word list:
///
/// ```
/// use gallry_puzzle_soulver::{Slot, WordGenerator};
///
/// // Create slots for a 3-letter word
/// let slots = vec![
///     Slot::new(vec!['c', 'b']),
///     Slot::new(vec!['a', 'o']),
///     Slot::new(vec!['t', 'r']),
/// ];
///
/// // Create a generator with the default embedded word list
/// let generator = WordGenerator::with_slots(slots);
///
/// // Print all valid words
/// for word in generator.iter() {
///     println!("{}", word);
/// }
/// ```
///
/// Using a custom word list:
///
/// ```
/// use gallry_puzzle_soulver::{Slot, WordGenerator};
/// use std::collections::HashSet;
///
/// // Create a custom word list
/// let word_list: HashSet<String> = vec!["cat".to_string(), "dog".to_string()]
///     .into_iter()
///     .collect();
///
/// // Create slots
/// let slots = vec![
///     Slot::new(vec!['c', 'd']),
///     Slot::new(vec!['a', 'o']),
///     Slot::new(vec!['t', 'g']),
/// ];
///
/// // Create generator with custom word list
/// let generator = WordGenerator::new(slots, Some(word_list));
///
/// // Get valid words
/// let valid_words: Vec<String> = generator.iter().collect();
///
/// // Should contain both "cat" and "dog"
/// assert_eq!(valid_words.len(), 2);
/// ```
pub struct WordGenerator {
    /// The slots defining character options for each position
    slots: Vec<Slot>,
    /// Optional word list for filtering
    word_list: Option<HashSet<String>>,
}

/// An iterator that generates and filters words based on slot options
pub struct WordIter<'a> {
    generator: &'a WordGenerator,
    current_indices: Vec<usize>,
    slot_sizes: Vec<usize>,
    done: bool,
}

impl<'a> WordIter<'a> {
    fn new(generator: &'a WordGenerator) -> Self {
        let slot_sizes: Vec<_> = generator.slots
            .iter()
            .map(|slot| slot.options.len())
            .collect();
            
        let has_options = slot_sizes.iter().all(|&size| size > 0);
        
        Self {
            generator,
            current_indices: vec![0; generator.slots.len()],
            slot_sizes,
            done: !has_options,
        }
    }
    
    fn build_word(&self) -> String {
        let mut word = String::with_capacity(self.current_indices.len());
        for (slot_idx, &char_idx) in self.current_indices.iter().enumerate() {
            word.push(self.generator.slots[slot_idx].options[char_idx]);
        }
        word
    }
    
    fn increment(&mut self) -> bool {
        for i in (0..self.current_indices.len()).rev() {
            self.current_indices[i] += 1;
            if self.current_indices[i] < self.slot_sizes[i] {
                return true;
            }
            // Reset this position and carry to next position
            self.current_indices[i] = 0;
        }
        // If we get here, we've overflowed
        self.done = true;
        false
    }
}

impl<'a> Iterator for WordIter<'a> {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        
        loop {
            let word = self.build_word();
            
            // Prepare for next iteration
            let has_next = self.increment();
            
            // Check if the word is in the dictionary
            if let Some(word_list) = &self.generator.word_list {
                if word_list.is_empty() || word_list.contains(&word) {
                    return Some(word);
                }
                
                // Not in the dictionary, continue if we have more words
                if !has_next {
                    return None;
                }
            } else {
                // No filtering, return all words
                return Some(word);
            }
        }
    }
}

/// An iterator that yields all possible combinations without filtering
pub struct AllCombinationsIter<'a> {
    slots: &'a [Slot],
    current_indices: Vec<usize>,
    slot_sizes: Vec<usize>,
    done: bool,
}

impl<'a> AllCombinationsIter<'a> {
    fn new(slots: &'a [Slot]) -> Self {
        let slot_sizes: Vec<_> = slots
            .iter()
            .map(|slot| slot.options.len())
            .collect();
            
        let has_options = slot_sizes.iter().all(|&size| size > 0);
        
        Self {
            slots,
            current_indices: vec![0; slots.len()],
            slot_sizes,
            done: !has_options,
        }
    }
    
    fn build_word(&self) -> String {
        let mut word = String::with_capacity(self.current_indices.len());
        for (slot_idx, &char_idx) in self.current_indices.iter().enumerate() {
            word.push(self.slots[slot_idx].options[char_idx]);
        }
        word
    }
    
    fn increment(&mut self) -> bool {
        for i in (0..self.current_indices.len()).rev() {
            self.current_indices[i] += 1;
            if self.current_indices[i] < self.slot_sizes[i] {
                return true;
            }
            // Reset this position and carry to next position
            self.current_indices[i] = 0;
        }
        // If we get here, we've overflowed
        self.done = true;
        false
    }
}

impl<'a> Iterator for AllCombinationsIter<'a> {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        
        let word = self.build_word();
        self.increment();
        Some(word)
    }
}

impl WordGenerator {
    /// Creates a new `WordGenerator` with the given slots and optional word list.
    ///
    /// If `word_list` is `None`, the generator will use the embedded default word list.
    ///
    /// # Parameters
    ///
    /// * `slots` - A vector of `Slot`s defining character options for each position
    /// * `word_list` - An optional custom word list for filtering generated words
    ///
    /// # Examples
    ///
    /// ```
    /// use gallry_puzzle_soulver::{Slot, WordGenerator};
    /// use std::collections::HashSet;
    ///
    /// // Create a custom word list
    /// let word_list: HashSet<String> = vec!["cat".to_string()].into_iter().collect();
    ///
    /// // Create a generator with the custom word list
    /// let generator = WordGenerator::new(
    ///     vec![
    ///         Slot::new(vec!['c', 'd']),
    ///         Slot::new(vec!['a', 'o']),
    ///         Slot::new(vec!['t', 'g']),
    ///     ],
    ///     Some(word_list),
    /// );
    /// ```
    pub fn new(slots: Vec<Slot>, word_list: Option<HashSet<String>>) -> Self {
        let word_list = match word_list {
            Some(list) => Some(list),
            None => {
                // Use the embedded wordlist
                let word_set: HashSet<String> = EMBEDDED_WORDLIST
                    .lines()
                    .map(|line| line.to_string())
                    .collect();

                Some(word_set)
            }
        };

        Self {
            slots,
            word_list,
        }
    }

    /// Creates a `WordGenerator` with the given slots and the default embedded word list.
    ///
    /// This is a convenience method equivalent to calling `new(slots, None)`.
    ///
    /// # Parameters
    ///
    /// * `slots` - A vector of `Slot`s defining character options for each position
    ///
    /// # Examples
    ///
    /// ```
    /// use gallry_puzzle_soulver::{Slot, WordGenerator};
    ///
    /// let generator = WordGenerator::with_slots(vec![
    ///     Slot::new(vec!['c', 'd']),
    ///     Slot::new(vec!['a', 'o']),
    ///     Slot::new(vec!['t', 'g']),
    /// ]);
    /// ```
    pub fn with_slots(slots: Vec<Slot>) -> Self {
        Self::new(slots, None)
    }

    /// Creates a `WordGenerator` with the given slots and an empty word list.
    ///
    /// With an empty word list, no filtering will be applied, so all possible
    /// word combinations will be returned by the iterator.
    ///
    /// # Parameters
    ///
    /// * `slots` - A vector of `Slot`s defining character options for each position
    ///
    /// # Examples
    ///
    /// ```
    /// use gallry_puzzle_soulver::{Slot, WordGenerator};
    ///
    /// let generator = WordGenerator::with_no_filtering(vec![
    ///     Slot::new(vec!['c', 'd']),
    ///     Slot::new(vec!['a', 'o']),
    ///     Slot::new(vec!['t', 'g']),
    /// ]);
    /// ```
    pub fn with_no_filtering(slots: Vec<Slot>) -> Self {
        Self {
            slots,
            word_list: Some(HashSet::new()),
        }
    }

    /// Loads a custom word list from a file at runtime.
    ///
    /// This method is useful when you need to load different word lists
    /// without recompiling the application.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the word list file (one word per line)
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error if the file could not be read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gallry_puzzle_soulver::{Slot, WordGenerator};
    ///
    /// let mut generator = WordGenerator::with_slots(vec![
    ///     Slot::new(vec!['c', 'd']),
    ///     Slot::new(vec!['a', 'o']),
    ///     Slot::new(vec!['t', 'g']),
    /// ]);
    ///
    /// // Load a custom word list from a file
    /// match generator.load_word_list_from_file("custom_words.txt") {
    ///     Ok(_) => println!("Word list loaded successfully"),
    ///     Err(e) => eprintln!("Failed to load word list: {}", e),
    /// }
    /// ```
    pub fn load_word_list_from_file(&mut self, path: &str) -> Result<()> {
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read word list from {}", path))?;

        let word_set: HashSet<String> = content.lines().map(|line| line.to_string()).collect();

        self.word_list = Some(word_set);
        Ok(())
    }

    /// Returns an iterator over the valid words based on the slots and word list.
    ///
    /// This method generates words on-demand as the iterator is consumed, providing
    /// a zero-copy implementation until a word is actually returned.
    ///
    /// If no word list is set, or if the word list is empty, all possible word
    /// combinations will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use gallry_puzzle_soulver::{Slot, WordGenerator};
    ///
    /// let generator = WordGenerator::with_slots(vec![
    ///     Slot::new(vec!['c', 'd']),
    ///     Slot::new(vec!['a', 'o']),
    ///     Slot::new(vec!['t', 'g']),
    /// ]);
    ///
    /// // Get all valid words as a Vec
    /// let words: Vec<_> = generator.iter().collect();
    /// println!("Found {} valid words", words.len());
    /// ```
    pub fn iter(&self) -> WordIter<'_> {
        WordIter::new(self)
    }

    /// Returns an iterator over all possible combinations without filtering.
    ///
    /// This method is useful when you need access to all possible combinations,
    /// regardless of whether they exist in the word list.
    ///
    /// # Examples
    ///
    /// ```
    /// use gallry_puzzle_soulver::{Slot, WordGenerator};
    ///
    /// let generator = WordGenerator::with_slots(vec![
    ///     Slot::new(vec!['c', 'd']),
    ///     Slot::new(vec!['a', 'o']),
    /// ]);
    ///
    /// // Get all possible combinations
    /// let all_combinations: Vec<String> = generator.all_combinations().collect();
    /// println!("All possible combinations: {:?}", all_combinations);
    /// ```
    pub fn all_combinations(&self) -> AllCombinationsIter<'_> {
        AllCombinationsIter::new(&self.slots)
    }

    /// Updates the word list used for filtering.
    ///
    /// # Parameters
    ///
    /// * `word_list` - The new word list to use for filtering
    ///
    /// # Examples
    ///
    /// ```
    /// use gallry_puzzle_soulver::{Slot, WordGenerator};
    /// use std::collections::HashSet;
    ///
    /// let mut generator = WordGenerator::with_no_filtering(vec![
    ///     Slot::new(vec!['c', 'd']),
    ///     Slot::new(vec!['a', 'o']),
    ///     Slot::new(vec!['t', 'g']),
    /// ]);
    ///
    /// // Add a custom filter
    /// let custom_list: HashSet<String> = vec!["cat".to_string()].into_iter().collect();
    /// generator.set_word_list(custom_list);
    ///
    /// // Now only "cat" will be returned (if it exists in the combinations)
    /// let filtered_words: Vec<_> = generator.iter().collect();
    /// ```
    pub fn set_word_list(&mut self, word_list: HashSet<String>) {
        self.word_list = Some(word_list);
    }
}
