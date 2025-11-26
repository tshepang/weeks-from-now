use clap::Parser;
use jiff::Timestamp;
use std::time::Duration;

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    /// Number of weeks
    count: u64,
    /// Count backwards
    #[arg(long)]
    past: bool,
}

fn main() {
    let cli = Cli::parse();
    let now = Timestamp::now();
    let date = if cli.past {
        now.checked_sub(Duration::from_hours(24 * 7 * cli.count))
    } else {
        now.checked_add(Duration::from_hours(24 * 7 * cli.count))
    };
    match date {
        Ok(date) => println!("{}", date.strftime("%F")),
        Err(why) => eprintln!("Date not representable: {why}!"),
    };
}
