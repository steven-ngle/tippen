use std::{fs::File, path::Path};
use serde::Deserialize;
use rand::seq::IndexedRandom;

#[derive(Deserialize)]
struct Quote {
    quote: String,
    source: String,
}

fn load_quotes() ->  Vec<Quote> {
    let file_path = Path::new("quotes/quotes.json");
    let file = File::open(file_path).expect("The file containing the quotes could not be opened.");

    let quotes: Vec<Quote> = serde_json::from_reader(file).expect("The file containing the quotes has invalid data.");

    return quotes;
}

fn show_quote(quote: &Quote) {
    println!("{}", quote.quote);
    println!("Source: {}", quote.source);
}

fn calculate_wpm(characters: usize, time: f64) -> f64 {
        return (characters as f64 / 5.0) / (time / 60.0);
}

fn read_input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Input Error!");

    return input;
}

fn count_characters(text: &str) -> usize {
    let characters = text.chars().count();
    return characters;
}

fn main() {
    let hint: &str = "Type ':quit' to end this program";
    let quotes = load_quotes();
    let mut rng = rand::rng();

    loop {

        let quote = quotes.choose(&mut rng).expect("List of quotes is empty");
        
        show_quote(quote);

        println!("{hint}");

        println!("Type the quote:");
            
        let start = std::time::Instant::now();
        let input = read_input();
        let time = start.elapsed().as_secs_f64();
        let typed = input.trim_end();

        if typed == ":quit" {
            break;
        } else if typed == &quote.quote {
            println!("GOD SLAIN");
        } else {
            println!("YOU DIED");
        }

        println!("You took: {time:.2} seconds");
            
        let characters = count_characters(typed);
        let wpm = calculate_wpm(characters, time);
        println!("WPM: {wpm:.1}");
    }
}
