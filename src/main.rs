use chrono::{Duration, Local};
use clap::Parser;

/// Date of x weeks from now
#[derive(Parser)]
#[clap(version)]
struct Opt {
    /// Number of weeks
    count: i64,
    /// Count backwards
    #[clap(long)]
    past: bool,
}

fn main() {
    let today = Local::today().naive_local();
    let opt = Opt::parse();
    let date = if opt.past {
        today.checked_sub_signed(Duration::weeks(opt.count))
    } else {
        today.checked_add_signed(Duration::weeks(opt.count))
    };
    match date {
        Some(date) => println!("{}", date),
        None => eprintln!("Date not representable... overflow!"),
    };
}
