fn show_quote(quote: &str, source: &str) {
    println!("{quote}");
    println!("Source: {source}");
}

fn calculate_wpm(characters: usize, time: f64) -> f64 {
        return (characters as f64 / 5.0) / (time / 60.0);
}

fn main() {
    let hint: &str = "Type ':quit' to end this program";
    let quote: &str = "My purpose standeth unchanged. Those stripped of the Grace of Gold shall all meet death. In the embrace of Messmers flame.";
    let source: &str = "Elden Ring";
    loop {
        show_quote(quote, source);

        println!("{hint}");

        println!("Type the quote:");
            
        let start = std::time::Instant::now();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Input Error!");
        let time = start.elapsed().as_secs_f64();
        let typed = input.trim_end();

        if typed == ":quit" {
            break;
        } else if typed == quote {
            println!("GOD SLAIN");
        } else {
            println!("YOU DIED");
        }

        println!("You took: {time:.2} seconds");
            
        let characters = typed.chars().count();
        let wpm = calculate_wpm(characters, time);
        println!("WPM: {wpm:.1}");
    }
}
