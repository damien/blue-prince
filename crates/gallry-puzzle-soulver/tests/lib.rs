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
    let mut word_generator = WordGenerator::new(
        vec![
            Slot::new(vec!['c', 'b', 'r']),
            Slot::new(vec!['a', 'i', 'o']),
            Slot::new(vec!['t', 's', 'e']),
        ]
    );

    word_generator.generate();

    assert_eq!(word_generator.get_words(), Some(&vec![
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
