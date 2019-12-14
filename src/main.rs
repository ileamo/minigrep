use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Проблема с аргументами: {}", err);
        process::exit(1);
    });
    println!("Поиск {}\nВ файле {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Ошибка: {}", e);
        process::exit(1);
    };
}
