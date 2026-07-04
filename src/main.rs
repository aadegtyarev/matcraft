use clap::Parser;
use rand::seq::IndexedRandom;

/// CLI-генератор русских матерных выражений по морфологическим правилам.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Тема генерации (хуй, пизда, ебать, блядь)
    #[arg(short, long)]
    topic: String,
}

fn main() {
    let cli = Cli::parse();

    let words: &[&str] = match cli.topic.as_str() {
        "хуй" => &["хуй", "хуяк", "захуярить", "охуеть", "хуякнул"],
        "пизда" => &["пизда", "пиздец", "запиздить", "пиздануть", "распиздяй"],
        "ебать" => &["ебать", "заебать", "выебать", "наебать", "проебать"],
        "блядь" => &["блядь", "блядство", "заблядовать", "блядский"],
        _ => {
            eprintln!(
                "Ошибка: неизвестная тема '{}'. Доступные темы: хуй, пизда, ебать, блядь",
                cli.topic
            );
            std::process::exit(1);
        }
    };

    let mut rng = rand::rng();
    let word = words.choose(&mut rng).expect("word list is non-empty");
    println!("{word}");
}
