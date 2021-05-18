use chrono::{Duration, Local};
use structopt::StructOpt;

/// Date of x weeks from now
#[derive(StructOpt)]
struct Opt {
    /// Number of weeks
    count: i64,
    /// Count backwards
    #[structopt(long)]
    past: bool,
}

fn main() {
    let today = Local::today().naive_local();
    let opt = Opt::from_args();
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
