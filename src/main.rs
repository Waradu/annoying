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
    let mut loop_count: Option<u64> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => {
                if i + 1 < args.len() {
                    text = args[i + 1].clone();
                    text = text.replace("\\n", "\n");
                    i += 1;
                }
            }
            "--speed" => {
                if i + 1 < args.len() {
                    speed = args[i + 1].parse().unwrap_or(default_speed);
                    i += 1;
                }
            }
            "--prevent-exit" => {
                prevent_exit = true;
            }
            "--loop" => {
                if i + 1 < args.len() {
                    if args[i + 1].starts_with("--") {
                        loop_count = Some(0);
                    } else {
                        match args[i + 1].parse::<u64>() {
                            Ok(n) => {
                                loop_count = Some(n);
                                i += 1;
                            }
                            Err(_) => {
                                loop_count = Some(0);
                            }
                        }
                    }
                } else {
                    loop_count = Some(0);
                }
            }
            _ => {}
        }
        i += 1;
    }

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

    let delay = std::time::Duration::from_secs_f64(speed);

    let print_tokens = || {
        for token in &tokens {
            if token == "\n" {
                println!();
            } else {
                print!("{} ", token);
            }
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            std::thread::sleep(delay);
        }
        println!();
    };

    match loop_count {
        None => {
            print_tokens();
        },
        Some(0) => {
            loop {
                print_tokens();
            }
        },
        Some(n) => {
            for _ in 0..n {
                print_tokens();
            }
        },
    }
}
