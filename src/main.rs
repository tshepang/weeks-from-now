use chrono::{Duration, Local};
use clap::Parser;

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    /// Number of weeks
    count: i64,
    /// Count backwards
    #[arg(long)]
    past: bool,
}

fn main() {
    let cli = Cli::parse();
    let today = Local::now().date_naive();
    let date = if cli.past {
        today.checked_sub_signed(Duration::weeks(cli.count))
    } else {
        today.checked_add_signed(Duration::weeks(cli.count))
    };
    match date {
        Some(date) => println!("{date}"),
        None => eprintln!("Date not representable... overflow!"),
    };
}
