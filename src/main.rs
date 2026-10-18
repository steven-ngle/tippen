fn main() {
    let quote: &str = "My purpose standeth unchanged. Those stripped of the Grace of Gold shall all meet death. In the embrace of Messmers flame."; 
    let source: &str = "Elden Ring";
    println!("{quote}");
    println!("Source: {source}");

    println!("Type the quote:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Input Error!");
    print!("{input}");

    if input.trim_end() == quote {
        println!("GOD SLAIN")
    } else {
        println!("YOU DIED")    
    }
}
