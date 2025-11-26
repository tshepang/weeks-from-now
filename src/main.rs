use clap::Parser;
use jiff::Timestamp;
use jiff::ToSpan;

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
    let now = Timestamp::now();
    let date = if cli.past {
        now.checked_sub((24 * 7).hours() * cli.count)
    } else {
        now.checked_add((24 * 7).hours() * cli.count)
    };
    match date {
        Ok(date) => println!("{}", date.strftime("%F")),
        Err(why) => eprintln!("Date not representable: {why}!"),
    }
}
