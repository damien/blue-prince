use anyhow::{Context, Result};
use argh::FromArgs;
use gallry_puzzle_soulver::{Slot, WordGenerator};

/// Finds possible words based on sets of allowed characters
#[derive(FromArgs)]
struct Args {
    /// character sets for each position (e.g., ABC DEF GHI)
    #[argh(positional)]
    char_sets: Vec<String>,

    /// optional path to a custom word list file
    #[argh(option, short = 'w')]
    word_list: Option<String>,

    /// show all combinations, even those not in the word list
    #[argh(switch, short = 'a')]
    all_combinations: bool,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    if args.char_sets.is_empty() {
        eprintln!("Error: You must provide at least one character set");
        std::process::exit(1);
    }

    // Convert each character set to a Slot
    let slots: Vec<Slot> = args.char_sets
        .iter()
        .map(|s| Slot::new(s.chars().collect()))
        .collect();

    // Create the appropriate generator based on arguments
    let mut generator = if args.all_combinations {
        WordGenerator::with_no_filtering(slots)
    } else {
        WordGenerator::with_slots(slots)
    };

    // Load custom word list if provided
    if let Some(path) = args.word_list {
        generator.load_word_list_from_file(&path)
            .with_context(|| format!("Failed to load word list from '{}'", path))?;
    }

    // Generate and display the words
    if args.all_combinations {
        for word in generator.all_combinations() {
            println!("{}", word);
        }
    } else {
        for word in generator.iter() {
            println!("{}", word);
        }
    }

    Ok(())
} 
