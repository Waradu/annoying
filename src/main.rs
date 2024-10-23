fn main() {
    let default_text = "Never gonna give you up
Never gonna let you down
Never gonna run around and desert you
Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you";
    let default_speed = 0.5;
    let args: Vec<String> = std::env::args().collect();
    let mut text = default_text.to_string();
    let mut speed = default_speed;
    let mut prevent_exit = false;

    for i in 0..args.len() {
        if args[i] == "--text" && i + 1 < args.len() {
            text = args[i + 1].clone();
            text = text.replace("\\n", "\n");
        }
        if args[i] == "--speed" && i + 1 < args.len() {
            speed = args[i + 1].parse().unwrap_or(default_speed);
        }
        if args[i] == "--prevent-exit" {
            prevent_exit = true;
        }
    }

    let delay = std::time::Duration::from_secs_f64(speed);

    let mut tokens = Vec::new();
    let mut current_word = String::new();

    for c in text.chars() {
        if c == '\n' {
            if !current_word.is_empty() {
                tokens.push(current_word.clone());
                current_word.clear();
            }
            tokens.push("\n".to_string());
        } else if c.is_whitespace() {
            if !current_word.is_empty() {
                tokens.push(current_word.clone());
                current_word.clear();
            }
        } else {
            current_word.push(c);
        }
    }
    if !current_word.is_empty() {
        tokens.push(current_word);
    }

    let _ = ctrlc::set_handler(move || {
        if !prevent_exit {
            std::process::exit(0);
        }
    });

    for token in tokens {
        if token == "\n" {
            println!();
            continue;
        } else {
            print!("{} ", token);
        }
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(delay);
    }
    println!();
}
