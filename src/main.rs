use std::path::PathBuf;
use std::fs;
use rand::seq::SliceRandom;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author="Leah Anderson", version, about="Rust program that randomly prints quotes")]
struct Args {
    #[arg(short, long, help = "Optionally specify a file to read quotes from instead of using the built-in quotes. Every line in the file is considered a quote.")]
    file: Option<PathBuf>,
    #[arg(short = 'F', long, action=ArgAction::SetFalse, help = "Choose whether to use fortune mode or not. Fortune mode parses the file as a custom fortune file.")]
    fortune: bool
}


fn main() {
    let args = Args::parse();

    let mut quotes: Vec<String> = vec![
        // Default quotes
        "People can be more understanding than you think they'll be. - Leah Anderson".to_string(),
        "It is during our darkest moments that we must focus to see the light. - Aristotle Onassis".to_string(),
        "No act of kindness, no matter how small, is ever wasted. - Aesop".to_string(),
        "Learn from yesterday, live for today, hope for tomorrow. The important thing is not to stop questioning. - Albert Einstein".to_string(),
        "Your work is going to fill a large part of your life, and the only way to be truly satisfied is to do what you believe is great work. And the only way to do great work is to love what you do. If you haven't found it yet, keep looking. Don't settle. As with all matters of the heart, you'll know when you find it. - Steve Jobs".to_string(),
        "Microsoft isn't evil, they just make really crappy operating systems. - Linus Torvalds".to_string(),
        "Software is like sex: it's better when it's free. - Linus Torvalds".to_string(),
        "You might not think that programmers are artists, but programming is an extremely creative profession. It's logic-based creativity. - John Romero".to_string(),
        "The memory management on the PowerPC can be used to frighten small children. - Linus Torvalds".to_string(),
        "If you start doing things because you hate others and want to screw them over, the end result is bad. - Linus Torvalds".to_string(),
        "The secret of getting ahead is getting started. - Mark Twain".to_string(),
        "Always code as if the guy who ends up maintaining your code will be a violent psychopath who knows where you live. - Martin Golding".to_string(),
        "Programming is like sex. One mistake and you have to support it for the rest of your life. - Michael Sinz".to_string(),
        "There are only two kinds of languages: the ones people complain about and the ones nobody uses. - Bjarne Stroustrup".to_string(),
        "Being a good software engineer is 3 percent talent, 97 percent not being distracted by the internet.".to_string(),
        "Code is like humor. When you have to explain it, it’s bad. – Cory House".to_string(),
        "In open source, we feel strongly that to really do something well, you have to get a lot of people involved. - Linus Torvalds".to_string(),
        "No matter how dark things seem, you have to keep going. There's a light at the ended of that tunnel, no matter what. - Leah Anderson".to_string(),
    ];

    if let Some(quotefile) = args.file {
        // Attempt to read the file
        let contents = fs::read_to_string(quotefile).expect("Could not read file");
        // Checks if the fortune flag is enabled, if so, run in fortune mode, otherwise, run in newline parsing mode
        if args.fortune {
            quotes = contents.split('%').map(|quote| quote.trim().to_string()).collect();
        }
        else {
            quotes = contents.lines().map(|line| line.to_string()).collect();
        }
    }
    // print a quote from the vector out, if there are no quotes, print an error message
    println!("{}", quotes.choose(&mut rand::thread_rng()).unwrap_or(&"No quotes found! Is the custom file you're using empty?".to_string()));
}
