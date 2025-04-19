use std::collections::HashSet;
use gallry_puzzle_soulver::{Slot, WordGenerator};

#[test]
fn test_to_string() {
    let slot = Slot::new(vec!['a', 'b', 'c']);
    assert_eq!(slot.to_string(), "a");
}

#[test]
fn test_deref() {
    let slot = Slot::new(vec!['a', 'b', 'c']);
    assert_eq!(*slot, 'a');
}

#[test]
fn test_iterator() {
    let slot = Slot::new(vec!['a', 'b', 'c']);
    assert_eq!(slot.collect::<Vec<_>>(), vec!['a', 'b', 'c']);
}

#[test]
fn test_generate() {
    // Use with_no_filtering to test without word filtering
    let mut word_generator = WordGenerator::with_no_filtering(
        vec![
            Slot::new(vec!['c', 'b', 'r']),
            Slot::new(vec!['a', 'i', 'o']),
            Slot::new(vec!['t', 's', 'e']),
        ]
    );

    word_generator.generate();

    let expected_words = vec![
        "cat".to_string(), "cas".to_string(), "cae".to_string(),
        "cit".to_string(), "cis".to_string(), "cie".to_string(),
        "cot".to_string(), "cos".to_string(), "coe".to_string(),
        "bat".to_string(), "bas".to_string(), "bae".to_string(),
        "bit".to_string(), "bis".to_string(), "bie".to_string(),
        "bot".to_string(), "bos".to_string(), "boe".to_string(),
        "rat".to_string(), "ras".to_string(), "rae".to_string(),
        "rit".to_string(), "ris".to_string(), "rie".to_string(),
        "rot".to_string(), "ros".to_string(), "roe".to_string()
    ];
    
    // Convert iterator to Vec for comparison
    let generated_words = word_generator.get_words().unwrap().collect::<Vec<_>>();
    assert_eq!(generated_words, expected_words);
}

#[test]
fn test_get_words_with_filtering() {
    // Create a list of allowed words
    let word_list: HashSet<String> = [
        "cat".to_string(), 
        "bot".to_string(),
        "rie".to_string(),
    ].into_iter().collect();
    
    // Create a generator with custom word list
    let mut word_generator = WordGenerator::new(
        vec![
            Slot::new(vec!['c', 'b', 'r']),
            Slot::new(vec!['a', 'i', 'o']),
            Slot::new(vec!['t', 's', 'e']),
        ],
        Some(word_list.clone())
    );
    
    // Generate all possible words
    word_generator.generate();
    
    // Only words in the word list should be returned
    // Convert to sorted Vec for predictable comparison
    let mut generated_words = word_generator.get_words().unwrap().collect::<Vec<_>>();
    generated_words.sort();
    
    let mut expected_words = word_list.into_iter().collect::<Vec<_>>();
    expected_words.sort();
    
    assert_eq!(generated_words, expected_words);
}

#[test]
fn test_embedded_wordlist() {
    // Use default constructor with embedded wordlist
    let mut word_generator = WordGenerator::with_slots(
        vec![
            Slot::new(vec!['c', 'b', 'r']),
            Slot::new(vec!['a', 'i', 'o']),
            Slot::new(vec!['t', 's', 'e']),
        ]
    );
    
    word_generator.generate();
    
    // Get filtered words
    let words = word_generator.get_words().unwrap().collect::<Vec<_>>();
    
    // Test that common words like "cat" are included in our filtered results
    // but uncommon combinations are filtered out
    assert!(words.contains(&"cat".to_string()), "Embedded wordlist should include 'cat'");
    assert!(words.contains(&"bat".to_string()), "Embedded wordlist should include 'bat'");
    
    // These words should be filtered out if not in the wordlist
    let non_words = word_generator.get_all_words().unwrap().iter()
        .filter(|w| !words.contains(w))
        .cloned()
        .collect::<Vec<_>>();
    
    assert!(!non_words.is_empty(), "Some generated words should be filtered out by the wordlist");
}
