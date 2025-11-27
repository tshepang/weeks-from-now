use clap::Parser;
use jiff::ToSpan;
use jiff::Zoned;

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
    let now = Zoned::now();
    let date = if cli.past {
        now - cli.count.week()
    } else {
        now + cli.count.week()
    };
    println!("{}", date.strftime("%F"));
}
