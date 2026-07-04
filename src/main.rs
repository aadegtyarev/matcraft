use clap::Parser;

mod engine;

#[derive(Parser)]
#[command(version, about = "CLI-генератор матерных выражений по теме")]
struct Cli {
    /// Тема для генерации — любое слово или фраза (например, птички, космос, философия)
    #[arg(short, long, required = true)]
    topic: String,

    /// Количество фраз для генерации (по умолчанию 1)
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn main() {
    let cli = Cli::parse();

    if cli.topic.trim().is_empty() {
        eprintln!("Ошибка: тема не указана. Используйте --topic <ТЕМА>");
        std::process::exit(1);
    }

    let count = cli.count.clamp(1, 100);
    let phrases = engine::generate(&cli.topic, count);

    for phrase in phrases {
        println!("{phrase}");
    }
}
