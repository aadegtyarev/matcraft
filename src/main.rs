use clap::{Parser, Subcommand};

mod engine;

#[derive(Parser)]
#[command(
    version,
    about = "CLI-инструмент для исследования морфологической парадигмы русского мата"
)]
struct Cli {
    /// Режим отображения корней: classic (9 корней) или full (все 35)
    #[arg(long, default_value_t = engine::morpheme::Mode::Classic)]
    mode: engine::morpheme::Mode,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Показать полную парадигму корня
    Explore {
        /// Корень для исследования (например, еб-)
        root: String,

        /// Фильтр по суффиксу (например, -ну-)
        #[arg(long)]
        suffix: Option<String>,
    },

    /// Сгенерировать случайные формы
    Generate {
        /// Корень для генерации (любой, если не указан)
        #[arg(long)]
        root: Option<String>,

        /// Количество форм (по умолчанию 1, макс. 100)
        #[arg(long, default_value_t = 1)]
        count: usize,
    },

    /// Список доступных корней
    #[command(name = "list-roots")]
    ListRoots,

    /// Случайный корень с лингвистической заметкой
    Random,

    /// Детерминированный «корень дня»: один и тот же корень в пределах суток
    #[command(name = "root-of-the-day")]
    RootOfTheDay,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Explore { root, suffix } => {
            // Normalise: strip trailing hyphen if present
            let root_name = root.strip_suffix('-').unwrap_or(&root);

            match engine::explore(root_name, suffix.as_deref()) {
                Ok(result) => {
                    let output = engine::display::format_explore(&result);
                    println!("{output}");
                }
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Generate { root, count } => {
            let root_name = root.as_deref().map(|r| r.strip_suffix('-').unwrap_or(r));

            if count > 100 {
                eprintln!("Ошибка: количество не может превышать 100.");
                std::process::exit(1);
            }

            // Validate root if provided
            if let Some(r) = root_name {
                if engine::roots::root_data(r).is_none() {
                    eprintln!("Ошибка: корень '{}' не найден.", r);
                    std::process::exit(1);
                }
            }

            let forms = engine::generate(cli.mode, root_name, count);
            for form in forms {
                println!("{form}");
            }
        }

        Commands::ListRoots => {
            let output = engine::display::format_list_roots(cli.mode);
            println!("{output}");
        }

        Commands::Random => {
            let rd = engine::random_root(cli.mode);
            let samples = engine::sample_forms(rd);
            let sample_refs: Vec<&str> = samples.iter().map(|s| s.as_str()).collect();
            let output = engine::display::format_random(rd, &sample_refs);
            println!("{output}");
        }

        Commands::RootOfTheDay => {
            // Only impure part: today's UTC day index from the system clock
            // (seconds since the epoch / 86_400). "Clock before epoch" is
            // impossible in practice; fall back to index 0 — still deterministic.
            let day_index = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() / 86_400)
                .unwrap_or(0);
            let rd = engine::root_of_the_day(cli.mode, day_index);
            let samples = engine::sample_forms(rd);
            let sample_refs: Vec<&str> = samples.iter().map(|s| s.as_str()).collect();
            let output = engine::display::format_random(rd, &sample_refs);
            println!("{output}");
        }
    }
}
