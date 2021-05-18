use chrono::{Duration, Local};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "weeks-from-now", about = "Date of x weeks from now")]
struct Opt {
    #[structopt(help = "Number of weeks")]
    count: i64,
    #[structopt(long = "past", help = "Count backwards")]
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
