struct Quote {
    text: String,
    source: String,
}

fn show_quote(quote: &Quote) {
    println!("{}", quote.text);
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

    let quote = Quote {
        text: String::from("My purpose standeth unchanged. Those stripped of the Grace of Gold shall all meet death. In the embrace of Messmers flame."),
        source: String::from("Elden Ring"),
    };

    loop {
        show_quote(&quote);

        println!("{hint}");

        println!("Type the quote:");
            
        let start = std::time::Instant::now();
        let input = read_input();
        let time = start.elapsed().as_secs_f64();
        let typed = input.trim_end();

        if typed == ":quit" {
            break;
        } else if typed == &quote.text {
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
